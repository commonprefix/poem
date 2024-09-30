use std::{collections::VecDeque, f64::consts::LN_2};

use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Exp};

#[derive(Debug, Clone, PartialEq)]
struct Progress {
    timestamp: f64,
    work: f64,
}

type Sample = Vec<Progress>;
type Samples = Vec<Sample>;

const INF: f64 = f64::INFINITY;

fn sample_honest(g: f64, max_weight: f64, mut get_work: impl FnMut() -> f64) -> Sample {
    let mut weight_improvements = vec![Progress {
        timestamp: 0.0,
        work: 0.0,
    }];
    let mut block_time = 0.0;
    let mut heaviest_chain_weight: f64 = 0.0;
    let mut receive_events: VecDeque<(f64, f64)> = VecDeque::new();

    let exponential_distribution = Exp::new(g).unwrap();
    let mut rng = rand::thread_rng();

    while weight_improvements.last().unwrap().work < max_weight {
        block_time += exponential_distribution.sample(&mut rng);

        // Before processing the newly mined block first process all received blocks before it
        while let Some((arrival_time, weight)) = receive_events.front() {
            if *arrival_time > block_time {
                break;
            }
            heaviest_chain_weight = heaviest_chain_weight.max(*weight);
            receive_events.pop_front();
        }

        let block_arrival_time = block_time + 1.0; // Δ = 1
        let this_block_weight = get_work();
        let new_chain_weight = heaviest_chain_weight + this_block_weight;
        receive_events.push_back((block_arrival_time, new_chain_weight)); // the optimal adversary delays as much as allowed

        if weight_improvements.last().unwrap().work < new_chain_weight {
            weight_improvements.push(Progress {
                timestamp: block_time,
                work: new_chain_weight,
            });
        }
    }

    weight_improvements
}

fn sample_adversary(g: f64, max_weight: f64, mut get_work: impl FnMut() -> f64) -> Sample {
    let mut block_time = 0.0;
    let mut block_weight = 0.0;

    let mut weight_improvements = vec![Progress {
        timestamp: 0.0,
        work: 0.0,
    }];

    let exponential_distribution = Exp::new(g).unwrap();
    let mut rng = rand::thread_rng();

    while weight_improvements.last().unwrap().work < max_weight {
        block_time += exponential_distribution.sample(&mut rng);
        block_weight += get_work();
        weight_improvements.push(Progress {
            timestamp: block_time,
            work: block_weight,
        });
    }

    return weight_improvements;
}

fn sample_multiple_adversaries(
    g: f64,
    max_weight: f64,
    mut get_work: impl FnMut() -> f64,
    monte_carlo: usize,
) -> Samples {
    let mut samples = Vec::new();
    for _i in 0..monte_carlo {
        samples.push(sample_adversary(g, max_weight, &mut get_work));
    }
    samples
}

fn entropy_function(gamma: f64) -> impl FnMut() -> f64 {
    let exponential_distribution = Exp::new(std::f64::consts::LN_2).unwrap();
    let mut rng = rand::thread_rng();

    move || exponential_distribution.sample(&mut rng) + gamma
}

fn sample_multiple_honest(
    g: f64,
    max_weight: f64,
    mut get_work: impl FnMut() -> f64,
    monte_carlo: usize,
) -> Samples {
    let mut samples = Vec::new();
    for _i in 0..monte_carlo {
        samples.push(sample_honest(g, max_weight, &mut get_work));
    }
    samples
}

fn get_k(honest_sample: Sample, adversary_sample: Sample) -> Option<f64> {
    if honest_sample.last().unwrap().timestamp >= adversary_sample.last().unwrap().timestamp {
        return Some(INF);
    }

    let mut j = adversary_sample.len() - 1;
    for i in (1..honest_sample.len()).rev() {
        while honest_sample[i].timestamp < adversary_sample[j - 1].timestamp {
            j -= 1;
        }
        if honest_sample[i - 1].work <= adversary_sample[j - 1].work {
            return Some(honest_sample[i].work);
        }
    }

    return None;
}

fn get_latency(honest_samples: Samples, adversary_samples: Samples, epsilon: f64) -> f64 {
    let mut potential_k = vec![];
    let mut f = 0.0;
    let monte_carlo = honest_samples.len();
    for i in 0..monte_carlo {
        potential_k.push(get_k(honest_samples[i].clone(), adversary_samples[i].clone()).unwrap());
        f += honest_samples[i].last().unwrap().work / honest_samples[i].last().unwrap().timestamp;
    }
    potential_k.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let k = potential_k[(monte_carlo as f64 * epsilon).ceil() as usize];
    f = f / monte_carlo as f64;
    let latency = k / f;

    latency
}

fn get_poem_latency_for_beta_g_gamma(beta: f64, g: f64, gamma: f64, monte_carlo: usize) -> f64 {
    let mut get_entropy = entropy_function(gamma);
    let entropy_mean = 1 as f64 / std::f64::consts::LN_2 + gamma;
    let max_weight = 400 as f64 * entropy_mean;
    // println!("gamma: {}", entropy_function(gamma)());
    let honest_samples = sample_poem_honest_for_g_gamma(g, gamma, monte_carlo);
    let adversary_samples = adjust_poem_adversary_g_gamma(
        &sample_multiple_adversaries(1.0, max_weight, get_entropy, monte_carlo),
        g * beta / (1.0 - beta),
        gamma,
    );

    get_latency(honest_samples, adversary_samples, 0.1)
}

fn sample_poem_honest_for_g_gamma(g: f64, gamma: f64, monte_carlo: usize) -> Samples {
    let mut get_entropy = entropy_function(gamma);
    let entropy_mean = 1 as f64 / std::f64::consts::LN_2 + gamma;
    let max_weight = 400 as f64 * entropy_mean;
    sample_multiple_honest(g, max_weight, &mut get_entropy, monte_carlo)
}

// fn adjust_poem_adversary_g_gamma(adversary_samples: &Samples, g: f64, gamma: f64) -> Samples {
//     adversary_samples.iter().map(|sample| {
//         sample.first().cloned().into_iter().chain(sample.iter().skip(1).map(|progress| {
//             Progress { timestamp: progress.timestamp / g, work: progress.work + gamma }
//         })).collect()
//     }).collect()
// }

fn adjust_poem_adversary_g_gamma(adversary_samples: &Samples, g: f64, gamma: f64) -> Samples {
    adversary_samples
        .iter()
        .map(|sample| {
            sample
                .first()
                .cloned()
                .into_iter()
                .chain(sample.iter().skip(1).map(|progress| Progress {
                    timestamp: progress.timestamp / g,
                    work: progress.work + gamma,
                }))
                .collect()
        })
        .collect()
}

fn get_bitcoin_latency_for_beta_g(beta: f64, g: f64, monte_carlo: usize) -> f64 {
    let max_weight = 400.0;
    let honest_samples = sample_multiple_honest(g, max_weight, || 1.0, monte_carlo);
    let adversary_samples =
        sample_multiple_adversaries(g * beta / (1.0 - beta), 400.0, || 1.0, monte_carlo);

    get_latency(honest_samples, adversary_samples, 0.1)
}

fn ternary_search(left: f64, right: f64, function_to_minimize: impl Fn(f64) -> f64) -> f64 {
    let inner_left = left + (right - left) / 3 as f64;
    let inner_right = right - (right - left) / 3 as f64;

    let left_value = function_to_minimize(inner_left);
    let right_value = function_to_minimize(inner_right);

    // println!("{inner_left} : {inner_right} -> {left_value} : {right_value}");
    if inner_right - inner_left < 0.1 {
        return (left_value + right_value) / 2.0;
    }

    if left_value < right_value {
        return ternary_search(left, inner_right, function_to_minimize);
    } else {
        return ternary_search(inner_left, right, function_to_minimize);
    }
}

fn get_poem_ideal_gamma_for_beta_g(beta: f64, g: f64, monte_carlo: usize) -> f64 {
    let function_to_minimize =
        |gamma: f64| -> f64 { get_poem_latency_for_beta_g_gamma(beta, g, gamma, monte_carlo) };

    return ternary_search(0.0, 10.0, function_to_minimize);
}

// fn get_poem_ideal_latencies_for_beta(beta: f64, g_range: Vec<f64> , monte_carlo: usize) -> Vec<(f64, f64)> {

//     for g in g_range.iter() {
//         let ideal_ =
//     }
//     todo!()
// }

fn main() {
    let start = std::time::Instant::now();

    let mut g_range: Vec<f64> = (0..=30).map(|x| x as f64 * 0.1).collect();
    let mut beta_range: Vec<f64> = (0..=40).map(|x| x as f64 * 0.01).collect();
    let mut gamma_range: Vec<f64> = (0..=50).map(|x| x as f64).collect();

    let monte_carlo = 1000;
    // // sample honest
    // for g in g_range.iter() {
    //     println!("working for g: {}", g);
    //     for gamma in gamma_range.iter() {
    //         let honest_samples = sample_poem_honest_for_g_gamma(*g, *gamma, monte_carlo);
    //     }
    // }

    // sample adversary
    let adversary_samples =
        sample_multiple_adversaries(1.0, 400.0, entropy_function(0.0), monte_carlo);
    for beta in beta_range.iter() {
        println!("adjusting beta: {}", beta);
        for g in g_range.iter() {
            for gamma in gamma_range.iter() {
                let a = adjust_poem_adversary_g_gamma(
                    &adversary_samples,
                    *g * *beta / (1.0 - *beta),
                    *gamma,
                );
            }
        }
    }

    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_poem_adversary() {
        let adversary_samples: Samples = vec![
            vec![
                Progress {
                    timestamp: 0.0,
                    work: 0.0,
                },
                Progress {
                    timestamp: 1.0,
                    work: 1.0,
                },
                Progress {
                    timestamp: 2.0,
                    work: 2.0,
                },
            ],
            vec![
                Progress {
                    timestamp: 0.0,
                    work: 0.0,
                },
                Progress {
                    timestamp: 2.0,
                    work: 1.0,
                },
                Progress {
                    timestamp: 3.0,
                    work: 2.0,
                },
            ],
        ];

        let modified_adversary_samples = adjust_poem_adversary_g_gamma(&adversary_samples, 0.5, 2.0);
        assert_eq!(
            modified_adversary_samples,
            vec![
                vec![
                    Progress {
                        timestamp: 0.0,
                        work: 0.0,
                    },
                    Progress {
                        timestamp: 2.0,
                        work: 3.0,
                    },
                    Progress {
                        timestamp: 4.0,
                        work: 4.0,
                    },
                ],
                vec![
                    Progress {
                        timestamp: 0.0,
                        work: 0.0,
                    },
                    Progress {
                        timestamp: 4.0,
                        work: 3.0,
                    },
                    Progress {
                        timestamp: 6.0,
                        work: 4.0,
                    },
                ],
            ]
        );
    }

    #[test]
    fn test_get_k() {
        let honest_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 2.0,
                work: 1.0,
            },
            Progress {
                timestamp: 3.0,
                work: 2.0,
            },
        ];
        let adversary_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 1.0,
                work: 1.0,
            },
            Progress {
                timestamp: 4.0,
                work: 2.0,
            },
        ];
        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), 2.0);

        let honest_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 1.0,
                work: 2.0,
            },
            Progress {
                timestamp: 4.0,
                work: 3.0,
            },
        ];
        let adversary_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 2.0,
                work: 1.0,
            },
            Progress {
                timestamp: 3.0,
                work: 3.0,
            },
        ];
        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), INF);

        let honest_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 1.0,
                work: 1.0,
            },
            Progress {
                timestamp: 3.0,
                work: 2.0,
            },
        ];
        let adversary_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 2.0,
                work: 1.0,
            },
            Progress {
                timestamp: 4.0,
                work: 2.0,
            },
        ];

        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), 2.0);

        let honest_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 1.0,
                work: 1.0,
            },
            Progress {
                timestamp: 3.0,
                work: 3.0,
            },
        ];
        let adversary_sample = vec![
            Progress {
                timestamp: 0.0,
                work: 0.0,
            },
            Progress {
                timestamp: 2.0,
                work: 0.5,
            },
            Progress {
                timestamp: 4.0,
                work: 2.0,
            },
        ];

        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), 1.0);
    }
}
