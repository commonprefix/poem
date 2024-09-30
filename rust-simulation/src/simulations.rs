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
        let max_work =
            adversary_sample.last().unwrap().work + gamma * (adversary_sample.len() as f64 - 1.0);

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

        let mut local_f = f_mutex.lock().unwrap();
        *local_f += latest_weight_improvement.work / latest_weight_improvement.timestamp;
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

fn get_poem_optimal_gamma_for_latency(
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

    gamma_range
        .into_iter()
        .map(|gamma| {
            println!("PoEM gamma: {gamma}");
            g_range
                .clone()
                .into_iter()
                .fold(vec![INF; beta_range.len()], |min_latency_g, g| {
                    let latency_g = get_latency(
                        g,
                        gamma,
                        get_poem_work,
                        &poem_adversary_samples,
                        &beta_range,
                        epsilon,
                    );

                    min_latency_g
                        .iter()
                        .zip(latency_g.iter())
                        .map(|(&a, &b)| a.min(b))
                        .collect()
                })[0]
        })
        .collect()
}

fn get_optimal_poem_parameters(
    monte_carlo: i32,
    epsilon: f64,
    g_range: Vec<f64>,
    beta_range: Vec<f64>,
    gamma_range: Vec<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let poem_adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_poem_work, &mut rng)
        })
        .collect();

    let mut optimal_g = vec![INF; beta_range.len()];
    let mut optimal_gamma = vec![INF; beta_range.len()];
    let mut optimal_latency = vec![INF; beta_range.len()];
    for g in &g_range {
        println!("working on g: {g}");
        for gamma in &gamma_range {
            println!(" - working on gamma: {gamma}");
            let latency = get_latency(
                *g,
                *gamma,
                get_poem_work,
                &poem_adversary_samples,
                &beta_range,
                epsilon,
            );

            for (i, &lat) in latency.iter().enumerate() {
                if lat < optimal_latency[i] {
                    optimal_g[i] = *g;
                    optimal_gamma[i] = *gamma;
                    optimal_latency[i] = lat;
                }
            }
        }
    }

    (optimal_g, optimal_gamma, optimal_latency)
}

fn optimal_poem() {
    let monte_carlo = 10000;
    let epsilon = 0.1;

    let g_range: Vec<f64> = (1..=30).map(|x| x as f64 * 0.1).collect();
    let mut gamma_range: Vec<f64> = (1..=49).map(|x| x as f64 * 0.02).collect();
    gamma_range.extend((1..=20).map(|x| x as f64).collect::<Vec<f64>>());
    let beta_range: Vec<f64> = (1..=40).map(|x| x as f64 * 0.01).collect();

    let optimal_poem_parameters = get_optimal_poem_parameters(
        monte_carlo,
        epsilon,
        g_range.clone(),
        beta_range.clone(),
        gamma_range.clone(),
    );

    println!("beta: {:?}", beta_range);
    println!("g: {:?}", optimal_poem_parameters.0);
    println!("gamma: {:?}", optimal_poem_parameters.1);
    println!("latency: {:?}", optimal_poem_parameters.2);
}

fn poem_vs_bitcoin() {
    let monte_carlo = 10000;
    let epsilon = 0.1;

    let g_range: Vec<f64> = (1..=30).map(|x| x as f64 * 0.1).collect();
    let gamma_range: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    let beta_range: Vec<f64> = (1..=40).map(|x| x as f64 * 0.01).collect();

    let optimal_poem_parameters = get_optimal_poem_parameters(
        monte_carlo,
        epsilon,
        g_range.clone(),
        beta_range.clone(),
        gamma_range.clone(),
    );
    let bitcoin_latency =
        get_min_bitcoin_latency(monte_carlo, epsilon, g_range.clone(), beta_range.clone());

    println!("PoEM: {:?}", optimal_poem_parameters.2);
    println!("Bitcoin: {:?}", bitcoin_latency);
}

fn gamma_latency() {
    let monte_carlo = 10000;
    let epsilon = 0.1;

    let g_range = (1..=30).map(|x| x as f64 * 0.1).collect();
    let gamma_range: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    let beta_range = vec![0.3];

    let latency = get_poem_optimal_gamma_for_latency(
        monte_carlo,
        epsilon,
        g_range,
        beta_range,
        gamma_range.clone(),
    );
    println!("gamma: {:?}", gamma_range);
    println!("latency: {:?}", latency);
}

pub fn latency_per_gamma_fixed_g(
    monte_carlo: i32,
    epsilon: f64,
    g: f64,
    beta_range: Vec<f64>,
    gamma_range: Vec<f64>,
) -> Vec<Vec<f64>> {
    let poem_adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_poem_work, &mut rng)
        })
        .collect();

    let latency_per_gamma = gamma_range
        .into_iter()
        .map(|gamma| {
            println!("working on gamma: {gamma}");
            get_latency(
                g,
                gamma,
                get_poem_work,
                &poem_adversary_samples,
                &beta_range,
                epsilon,
            )
        })
        .collect();
    latency_per_gamma
}

pub fn optimal_gamma_fixed_g(
    monte_carlo: i32,
    epsilon: f64,
    g: f64,
    beta_range: Vec<f64>,
    gamma_range: Vec<f64>,
) -> (Vec<f64>, Vec<f64>) {
    let poem_adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_poem_work, &mut rng)
        })
        .collect();

    let mut optimal_gamma_per_beta = vec![INF; beta_range.len()];
    let mut optimal_latency_per_beta = vec![INF; beta_range.len()];

    for &gamma in &gamma_range {
        let latency_per_beta = get_latency(
            g,
            gamma,
            get_poem_work,
            &poem_adversary_samples,
            &beta_range,
            epsilon,
        );

        for (i, &lat) in latency_per_beta.iter().enumerate() {
            if lat < optimal_latency_per_beta[i] {
                optimal_gamma_per_beta[i] = gamma;
                optimal_latency_per_beta[i] = lat;
            }
        }
    }

    (optimal_gamma_per_beta, optimal_latency_per_beta)
}

pub fn optimal_gamma(monte_carlo: i32, epsilon: f64) {
    let g_range: Vec<f64> = (1..=30).map(|x| x as f64 * 0.1).collect();
    let gamma_range: Vec<f64> = (1..=50).map(|x| x as f64).collect();
    let beta = 0.18;

    let poem_adversary_samples: Samples = (0..monte_carlo)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            sample_adversary(1.0, 600.0, get_poem_work, &mut rng)
        })
        .collect();

    let ideal_gamma_per_g: Vec<f64> = g_range
        .clone()
        .into_iter()
        .map(|g| {
            println!("working on g: {g}");
            let mut optimal_latency = INF;
            let mut optimal_gamma = INF;
            for gamma in &gamma_range {
                let latency_gamma = get_latency(
                    g,
                    *gamma,
                    get_poem_work,
                    &poem_adversary_samples,
                    &vec![beta],
                    epsilon,
                );

                if latency_gamma[0] < optimal_latency {
                    optimal_latency = latency_gamma[0];
                    optimal_gamma = *gamma;
                }
            }
            optimal_gamma
        })
        .collect();

    println!("g: {:?}", g_range);
    println!("gamma: {:?}", ideal_gamma_per_g);
}

fn main() {
    let start = std::time::Instant::now();

    // gamma_latency();
    // poem_vs_bitcoin();
    // optimal_gamma(10000, 0.1);
    // optimal_poem();

    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}
