use rand::rngs::ThreadRng;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use float_ord::FloatOrd;
use rand_distr::{Distribution, Exp};

#[derive(Debug)]
struct Progress {
    timestamp: f64,
    work: f64,
}

type Sample = Vec<Progress>;
type Samples = Vec<Sample>;

const INF: f64 = f64::INFINITY;

fn sample_adversary<T: rand::Rng>(
    g: f64,
    max_weight: f64,
    get_work: fn(f64, Exp<f64>, &mut T) -> f64,
    mut rng: &mut T,
) -> Sample {
    let mut block_time = 0.0;
    let mut block_weight = 0.0;

    let mut weight_improvements = vec![Progress {
        timestamp: 0.0,
        work: 0.0,
    }];

    let time_distribution = Exp::new(g).unwrap();
    let work_distribution = Exp::new(std::f64::consts::LN_2).unwrap();

    while weight_improvements.last().unwrap().work < max_weight {
        block_weight += get_work(0.0, work_distribution, &mut rng);
        block_time += time_distribution.sample(&mut rng);
        weight_improvements.push(Progress {
            timestamp: block_time,
            work: block_weight,
        });
    }

    return weight_improvements;
}

fn transform_adversary(progress: &Progress, g: f64, beta: f64, gamma: f64) -> Progress {
    Progress {
        timestamp: progress.timestamp / (g * beta / (1.0 - beta)),
        work: if progress.work == 0.0 {
            0.0
        } else {
            progress.work + gamma
        },
    }
}

// Call this with different g and gamma values (by changing get_work() and transform_adversary(progress, beta))
fn get_latency(
    g: f64,
    gamma: f64,
    get_work: fn(f64, Exp<f64>, &mut ThreadRng) -> f64,
    adversary_samples: &Samples,
    beta_range: Vec<f64>,
    epsilon: f64,
) -> Vec<f64> {
    let time_distribution = Exp::new(g).unwrap();
    let work_distribution = Exp::new(std::f64::consts::LN_2).unwrap();
    let f_atomic = AtomicU64::new(0);
    let max_k_mutex: Arc<Mutex<Vec<BinaryHeap<Reverse<FloatOrd<f64>>>>>> =
        Arc::new(Mutex::new(vec![BinaryHeap::new(); beta_range.len()]));

    adversary_samples
        .par_iter()
        .for_each(|adversary_sample| {
            let mut rng = rand::thread_rng();
            let mut block_time = 0.0;
            let mut heaviest_chain_weight: f64 = 0.0;
            let mut receive_events: VecDeque<(f64, f64)> = VecDeque::new();
            let mut mem_adv_progress = vec![0; beta_range.len()];

            let mut previous_weight_improvement = Progress {
                timestamp: 0.0,
                work: 0.0,
            };

            let mut latest_weight_improvement = Progress {
                timestamp: 0.0,
                work: 0.0,
            };

            let mut k = vec![INF; beta_range.len()];
            let max_work = adversary_sample.last().unwrap().work + gamma;

            while latest_weight_improvement.work < max_work {
                block_time += time_distribution.sample(&mut rng);

                // Before processing the newly mined block first process all received blocks before it
                while let Some((arrival_time, weight)) = receive_events.front() {
                    if *arrival_time > block_time {
                        break;
                    }
                    heaviest_chain_weight = heaviest_chain_weight.max(*weight);
                    receive_events.pop_front();
                }

                let block_arrival_time = block_time + 1.0; // Δ = 1
                let this_block_weight = get_work(gamma, work_distribution, &mut rng);
                let new_chain_weight = heaviest_chain_weight + this_block_weight;
                receive_events.push_back((block_arrival_time, new_chain_weight)); // the optimal adversary delays as much as allowed

                if new_chain_weight <= latest_weight_improvement.work {
                    continue;
                }

                // honest heaviest chain grows
                previous_weight_improvement = latest_weight_improvement;

                latest_weight_improvement = Progress {
                    timestamp: block_time,
                    work: new_chain_weight,
                };
                // println!("Extended longest chain: {:?}", latest_weight_improvement);

                // ----

                for beta_index in 0..beta_range.len() {
                    let beta = beta_range[beta_index];

                    for j in mem_adv_progress[beta_index]..adversary_sample.len() {
                        let adv_progress = transform_adversary(&adversary_sample[j], g, beta, gamma);
                        if adv_progress.timestamp > latest_weight_improvement.timestamp {
                            // update k if needed
                            let prev_adv_progress = transform_adversary(&adversary_sample[j - 1], g, beta, gamma);
                            // println!("Found Adversary Progress with timestamp in front: {:?}", adv_progress);
                            // println!("Previous: {:?}", prev_adv_progress);
                            if prev_adv_progress.work >= previous_weight_improvement.work {
                                // found latest k
                                k[beta_index] = latest_weight_improvement.work;
                                // println!("Found latest k: {:?}", latest_weight_improvement.work);
                            }
                            if prev_adv_progress.work > latest_weight_improvement.work {
                                // adversary is ahead, no k found yet
                                k[beta_index] = INF;
                                // println!("Made k INF");
                            }
                            mem_adv_progress[beta_index] = j;
                            break;
                        }
                    }
                }
            }

            f_atomic.fetch_add((latest_weight_improvement.work / latest_weight_improvement.timestamp) as u64, Ordering::SeqCst);
            // f += latest_weight_improvement.work / latest_weight_improvement.timestamp;
            let mut temp_max_k = max_k_mutex.lock().unwrap();

            let error_count = (epsilon * adversary_samples.len() as f64).round() as usize;
            for beta_index in 0..beta_range.len() {
                if temp_max_k[beta_index].len() < error_count {
                    temp_max_k[beta_index].push(Reverse(FloatOrd(k[beta_index])));
                } else if let Some(&Reverse(smallest)) = temp_max_k[beta_index].peek() {
                    if k[beta_index] > smallest.0 {
                        temp_max_k[beta_index].pop();
                        temp_max_k[beta_index].push(Reverse(FloatOrd(k[beta_index])));
                    }
                }
            }
        });

    let f  = f_atomic.load(Ordering::SeqCst) as f64 / adversary_samples.len() as f64;

    let max_k = Arc::try_unwrap(max_k_mutex).unwrap().into_inner().unwrap();

    beta_range
        .iter()
        .enumerate()
        .map(|(beta_index, _)| max_k[beta_index].peek().unwrap().0 .0 / f)
        .collect()
}

fn get_poem_work<T: rand::Rng>(gamma: f64, work_distribution: Exp<f64>, rng: &mut T) -> f64 {
    work_distribution.sample(rng) + gamma
}

fn get_bitcoin_work(_: f64, _: f64, _: f64) -> f64 {
    1.0
}

fn main() {
    let start = std::time::Instant::now();
    let g = 0.4;
    let gamma = 0.5;

    let monte_carlo = 100000;

    let adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter() // Use Rayon’s parallel iterator
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_poem_work, &mut rng)
        })
        .collect();

    let beta_range = vec![0.4];
    let epsilon = 0.1;

    // let mut beta_range: Vec<f64> = (0..=40).map(|x| x as f64 * 0.01).collect();
    println!(
        "{:?}",
        get_latency(
            g,
            gamma,
            get_poem_work,
            &adversary_samples,
            beta_range,
            epsilon
        )
    );
    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}