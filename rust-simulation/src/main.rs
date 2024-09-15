use rand::rngs::ThreadRng;
use rayon::prelude::*;
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

fn transform_adversary(
    progress: &Progress,
    g: f64,
    beta: f64,
    gamma: f64,
    height: usize,
) -> Progress {
    Progress {
        timestamp: progress.timestamp / (g * beta / (1.0 - beta)),
        work: progress.work + (gamma * height as f64),
    }
}

fn get_latency(
    g: f64,
    gamma: f64,
    get_work: fn(f64, Exp<f64>, &mut ThreadRng) -> f64,
    adversary_samples: &Samples,
    beta_range: &Vec<f64>,
    epsilon: f64,
) -> Vec<f64> {
    let time_distribution = Exp::new(g).unwrap();
    let work_distribution = Exp::new(std::f64::consts::LN_2).unwrap();
    let f_mutex = Arc::new(Mutex::new(0.0));
    let max_k_mutex: Arc<Mutex<Vec<BinaryHeap<Reverse<FloatOrd<f64>>>>>> =
        Arc::new(Mutex::new(vec![BinaryHeap::new(); beta_range.len()]));

    adversary_samples.par_iter().for_each(|adversary_sample| {
        let mut rng = rand::thread_rng();
        let mut block_time = 0.0;
        let mut heaviest_chain_weight: f64 = 0.0;
        let mut receive_events: VecDeque<(f64, f64)> = VecDeque::new();
        let mut mem_adv_progress = vec![0; beta_range.len()];

        let mut previous_weight_improvement;
        let mut latest_weight_improvement = Progress {
            timestamp: 0.0,
            work: 0.0,
        };

        let mut k = vec![INF; beta_range.len()];
        let max_work = adversary_sample.last().unwrap().work + gamma * (adversary_sample.len() as f64 - 1.0);

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

            // update k if needed for each beta
            for beta_index in 0..beta_range.len() {
                let beta = beta_range[beta_index];

                for j in mem_adv_progress[beta_index]..adversary_sample.len() {
                    let adv_progress = transform_adversary(&adversary_sample[j], g, beta, gamma, j);
                    if adv_progress.timestamp > latest_weight_improvement.timestamp {
                        // update k if needed
                        let prev_adv_progress =
                            transform_adversary(&adversary_sample[j - 1], g, beta, gamma, j - 1);
                        if prev_adv_progress.work >= previous_weight_improvement.work {
                            // found latest k
                            k[beta_index] = latest_weight_improvement.work;
                        }
                        if prev_adv_progress.work > latest_weight_improvement.work {
                            // adversary is ahead, no k found yet
                            k[beta_index] = INF;
                        }
                        mem_adv_progress[beta_index] = j;
                        break;
                    }
                }
            }
        }

        let mut local_f = f_mutex.lock().unwrap();
        *local_f += latest_weight_improvement.work / latest_weight_improvement.timestamp;

        let mut local_max_k = max_k_mutex.lock().unwrap();

        let error_count = (epsilon * adversary_samples.len() as f64).round() as usize;
        for beta_index in 0..beta_range.len() {
            if local_max_k[beta_index].len() < error_count {
                local_max_k[beta_index].push(Reverse(FloatOrd(k[beta_index])));
            } else if let Some(&Reverse(smallest)) = local_max_k[beta_index].peek() {
                if k[beta_index] > smallest.0 {
                    local_max_k[beta_index].pop();
                    local_max_k[beta_index].push(Reverse(FloatOrd(k[beta_index])));
                }
            }
        }
    });

    let f =
        Arc::try_unwrap(f_mutex).unwrap().into_inner().unwrap() / adversary_samples.len() as f64;
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

fn get_bitcoin_work<T: rand::Rng>(_: f64, _: Exp<f64>, _: &mut T) -> f64 {
    1.0
}

fn get_min_bitcoin_latency(
    monte_carlo: i32,
    epsilon: f64,
    g_range: Vec<f64>,
    beta_range: Vec<f64>,
) -> Vec<f64> {
    let bitcoin_adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_bitcoin_work, &mut rng)
        })
        .collect();

    g_range
        .into_iter()
        .fold(vec![INF; beta_range.len()], |min_latency_g, g| {
            let latency_g = get_latency(
                g,
                0.0,
                get_bitcoin_work,
                &bitcoin_adversary_samples,
                &beta_range,
                epsilon,
            );

            min_latency_g
                .iter()
                .zip(latency_g.iter())
                .map(|(&a, &b)| a.min(b))
                .collect()
        })
}

fn get_min_poem_latency(
    monte_carlo: i32,
    epsilon: f64,
    g_range: Vec<f64>,
    beta_range: Vec<f64>,
    gamma_range: Vec<f64>,
) -> Vec<f64> {
    let poem_adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_poem_work, &mut rng)
        })
        .collect();

    g_range
        .clone()
        .into_iter()
        .fold(vec![INF; beta_range.len()], |min_latency_g, g| {
            println!("PoEM g: {g}");
            let latency_g = gamma_range.clone().into_iter().fold(
                vec![INF; beta_range.len()],
                |min_latency_gamma, gamma| {
                    println!(" - gamma: {gamma}");
                    let latency_gamma = get_latency(
                        g,
                        gamma,
                        get_poem_work,
                        &poem_adversary_samples,
                        &beta_range,
                        epsilon,
                    );

                    min_latency_gamma
                        .iter()
                        .zip(latency_gamma.iter())
                        .map(|(&a, &b)| a.min(b))
                        .collect()
                },
            );

            min_latency_g
                .iter()
                .zip(latency_g.iter())
                .map(|(&a, &b)| a.min(b))
                .collect()
        })
}

fn main() {
    let start = std::time::Instant::now();

    let monte_carlo = 10000;
    let epsilon = 0.1;

    let g_range: Vec<f64> = (1..=30).map(|x| x as f64 * 0.1).collect();
    let gamma_range: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    let beta_range: Vec<f64> = (1..=40).map(|x| x as f64 * 0.01).collect();

    let poem_latency = get_min_poem_latency(
        monte_carlo,
        epsilon,
        g_range.clone(),
        beta_range.clone(),
        gamma_range.clone(),
    );
    let bitcoin_latency =
        get_min_bitcoin_latency(monte_carlo, epsilon, g_range.clone(), beta_range.clone());

    println!("PoEM: {:?}", poem_latency);
    println!("Bitcoin: {:?}", bitcoin_latency);

    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}
