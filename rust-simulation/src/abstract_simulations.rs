use rand_distr::{Distribution, Exp};

const INF: f64 = f64::INFINITY;

#[derive(Debug, Clone, PartialEq, Copy)]
struct Chain {
    timestamp: f64,
    work: f64,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    timestamp: f64,
    work: f64,
}

fn sample_block_timestamps<T: rand::Rng, const N: usize>(g: f64, mut rng: &mut T) -> [f64; N] {
    let time_distribution = Exp::new(g).unwrap();
    let mut block_time = 0.0;
    let mut block_timestamps = [0.0; N];

    for i in 0..N {
        block_time += time_distribution.sample(&mut rng);
        block_timestamps[i] = block_time;
    }

    block_timestamps
}
fn get_bitcoin_blocks<const N: usize>(block_timestamps: [f64; N]) -> [Block; N] {
    block_timestamps.map(|t| Block {
        timestamp: t,
        work: 1.,
    })
}

fn get_work_progresses<const N: usize, const C: usize>(blocks: [Block; N], network_delay: f64) -> Vec<Chain> {
    let mut work_progresses = [None; C];
    work_progresses[0] = Some(Chain {
        timestamp: 0.0,
        work: 0.0,
        height: 0,
    });
    let mut work_progress_index = 0;

    // If there is no network delay, the blocks will be chained in series
    if network_delay == 0. {
        for block in blocks {
            work_progress_index += 1;
            work_progresses[work_progress_index] = Some(Chain {
                timestamp: block.timestamp,
                work: work_progresses[work_progress_index - 1].unwrap().work + block.work,
                height: work_progresses[work_progress_index - 1].unwrap().height + 1,
            });
        }
    } else {
        let mut arrival_events: [Option<(f64, Chain)>; N] = [None; N];
        let mut queue_front_index = 0;
        let mut queue_back_index = 0;

        let mut current_heaviest_chain = Chain {
            timestamp: 0.0,
            work: 0.0,
            height: 0,
        };

        for new_block in blocks {
            // Before processing the newly mined block first process all received blocks before it
            loop {
                let queue_front = arrival_events[queue_front_index];
                if queue_front.is_none() {
                    break;
                }
                let (arrival_time, chain) = queue_front.unwrap();
                if arrival_time > new_block.timestamp {
                    break;
                }
                if current_heaviest_chain.work < chain.work {
                    current_heaviest_chain = chain;
                }
                queue_front_index += 1;
            }

            let new_chain = Chain {
                timestamp: new_block.timestamp,
                work: current_heaviest_chain.work + new_block.work,
                height: current_heaviest_chain.height + 1,
            };

            if new_chain.work <= work_progresses[work_progress_index].unwrap().work {
                continue;
            }

            work_progress_index += 1;
            work_progresses[work_progress_index] = Some(new_chain);

            let new_chain_arrival_time = new_chain.timestamp + network_delay; // the optimal adversary delays as much as allowed
            arrival_events[queue_back_index] = Some((new_chain_arrival_time, new_chain));
            queue_back_index += 1;
        }
    }

    let mut vec = work_progresses.to_vec();
    vec.truncate(work_progress_index + 1);
    vec.iter().map(|x| x.unwrap()).collect()
}

fn scale_blocks<const N: usize>(blocks: &[Block; N], g: f64, gamma: f64) -> [Block; N] {
    blocks.map(|block| Block {
        timestamp: block.timestamp / g,
        work: block.work + gamma,
    })
}

fn get_k(honest_work_progresses: &Vec<Chain>, adversary_work_progresses: &Vec<Chain>) -> f64 {
    let mut k = INF;
    let mut adversary_index = 0;
    // Go through each honest progress
    for honest_index in 1..honest_work_progresses.len() {
        let honest_progress = honest_work_progresses[honest_index];
        // Find the adversary progress that immediately follows the honest progress in time
        while adversary_work_progresses[adversary_index].timestamp <= honest_progress.timestamp {
            adversary_index += 1;
            // If there is no adversary progress after the honest progress, we end the simulation here
            if adversary_index == adversary_work_progresses.len() {
                return k
            }
        }

        // update k if needed
        let previous_adversary_progress = adversary_work_progresses[adversary_index - 1];
        let previous_honest_progress = honest_work_progresses[honest_index - 1];

        if previous_adversary_progress.work >= previous_honest_progress.work {
            // found latest k
            k = honest_progress.work;
        }
        if previous_adversary_progress.work >= honest_progress.work {
            // adversary is ahead, no k found yet
            k = INF;
        }
    }
    k
}

pub fn one_sample_bitcoin_performance(g: f64, beta: f64) {
    const ADVERSARY_BLOCK_HEIGHT: usize = 600;
    const HONEST_BLOCK_HEIGHT: usize = 1800;

    const ADVERSARY_MAX_PROGRESS_HEIGHT: usize = 601;
    const HONEST_MAX_PROGRESS_HEIGHT: usize = 1801;

    let (honest_blocks, adversary_blocks) = sample_bitcoin::<HONEST_BLOCK_HEIGHT, ADVERSARY_BLOCK_HEIGHT>();
    // for _ in 0..100000 {
    let scaled_honest_blocks = scale_blocks(&honest_blocks, g, 0.);
    let scaled_adversary_blocks = scale_blocks(&adversary_blocks, g * beta / (1. - beta), 0.);

    let honest_work_progresses = get_work_progresses::<HONEST_BLOCK_HEIGHT, HONEST_MAX_PROGRESS_HEIGHT>(scaled_honest_blocks, 1.);
    let adversary_work_progresses = get_work_progresses::<ADVERSARY_BLOCK_HEIGHT, ADVERSARY_MAX_PROGRESS_HEIGHT>(scaled_adversary_blocks, 0.);
    // }

    let k = get_k(&honest_work_progresses, &adversary_work_progresses);
    let f = honest_work_progresses.last().unwrap().work / adversary_work_progresses.last().unwrap().timestamp;
    println!("f: {}", f);
    println!("k: {}", k);
}

fn sample_bitcoin<const HONEST_BLOCK_HEIGHT: usize, const ADVERSARY_BLOCK_HEIGHT: usize>() -> ([Block; HONEST_BLOCK_HEIGHT], [Block; ADVERSARY_BLOCK_HEIGHT]) {
    let mut rng = rand::thread_rng();
    let honest_block_timestamps = sample_block_timestamps::<_, HONEST_BLOCK_HEIGHT>(1.0, &mut rng);
    let adversary_block_timestamps = sample_block_timestamps::<_, ADVERSARY_BLOCK_HEIGHT>(1.0, &mut rng);

    let honest_blocks = get_bitcoin_blocks(honest_block_timestamps);
    let adversary_blocks = get_bitcoin_blocks(adversary_block_timestamps);
    (honest_blocks, adversary_blocks)
}

fn sample_poem_blocks<T: rand::Rng, const N: usize>(block_timestamps: [f64; N], work_distribution: Exp<f64>, rng: &mut T) -> [Block; N] {
    block_timestamps.map(|t| Block {
        timestamp: t,
        work: work_distribution.sample(rng),
    })
}


pub fn compare_bitcoin_and_poem_one_sample(g: f64, beta: f64, gamma: f64) {
    const ADVERSARY_BLOCK_HEIGHT: usize = 600;
    const HONEST_BLOCK_HEIGHT: usize = 1800;

    const ADVERSARY_MAX_PROGRESS_HEIGHT: usize = 601;
    const HONEST_MAX_PROGRESS_HEIGHT: usize = 1801;

    // Create common timestamps for both Bitcoin and Poem
    let mut rng = rand::thread_rng();
    let honest_block_timestamps = sample_block_timestamps::<_, HONEST_BLOCK_HEIGHT>(1.0, &mut rng);
    let adversary_block_timestamps = sample_block_timestamps::<_, ADVERSARY_BLOCK_HEIGHT>(1.0, &mut rng);

    // Create Bitcoin blocks
    let bitcoin_honest_blocks = get_bitcoin_blocks(honest_block_timestamps);
    let bitcoin_adversary_blocks = get_bitcoin_blocks(adversary_block_timestamps);

    // Sample Poem blocks
    let work_distribution = Exp::new(std::f64::consts::LN_2).unwrap();
    let poem_honest_blocks = sample_poem_blocks(honest_block_timestamps, work_distribution, &mut rng);
    let poem_adversary_blocks = sample_poem_blocks(adversary_block_timestamps, work_distribution,&mut rng);

    // Scale Bitcoin blocks
    let bitcoin_scaled_honest_blocks = scale_blocks(&bitcoin_honest_blocks, g, 0.);
    let bitcoin_scaled_adversary_blocks = scale_blocks(&bitcoin_adversary_blocks, g * beta / (1. - beta), 0.);

    // Scale Poem blocks
    let poem_scaled_honest_blocks = scale_blocks(&poem_honest_blocks, g, gamma);
    let poem_scaled_adversary_blocks = scale_blocks(&poem_adversary_blocks, g * beta / (1. - beta), gamma);

    // Get Bitcoin work progress
    let bitcoin_honest_work_progresses = get_work_progresses::<HONEST_BLOCK_HEIGHT, HONEST_MAX_PROGRESS_HEIGHT>(bitcoin_scaled_honest_blocks, 1.);
    let bitcoin_adversary_work_progresses = get_work_progresses::<ADVERSARY_BLOCK_HEIGHT, ADVERSARY_MAX_PROGRESS_HEIGHT>(bitcoin_scaled_adversary_blocks, 0.);

    // Get Poem work progress
    let poem_honest_work_progresses = get_work_progresses::<HONEST_BLOCK_HEIGHT, HONEST_MAX_PROGRESS_HEIGHT>(poem_scaled_honest_blocks, 1.);
    let poem_adversary_work_progresses = get_work_progresses::<ADVERSARY_BLOCK_HEIGHT, ADVERSARY_MAX_PROGRESS_HEIGHT>(poem_scaled_adversary_blocks, 0.);

    // Get Bitcoin k
    let bitcoin_k = get_k(&bitcoin_honest_work_progresses, &bitcoin_adversary_work_progresses);

    // Get Poem k
    let poem_k = get_k(&poem_honest_work_progresses, &poem_adversary_work_progresses);

    println!("Bitcoin k: {}", bitcoin_k);
    println!("Poem k: {}", poem_k);
}

// pub fn monte_carlo() {
//     const ADVERSARY_BLOCK_HEIGHT: usize = 600;
//     const HONEST_BLOCK_HEIGHT: usize = 1800;

//     const ADVERSARY_MAX_PROGRESS_HEIGHT: usize = 601;
//     const HONEST_MAX_PROGRESS_HEIGHT: usize = 1801;
//     let samples: Vec<(_, _)> = (0..100).map(|_| sample_bitcoin::<HONEST_BLOCK_HEIGHT, ADVERSARY_BLOCK_HEIGHT>()).collect();

//     for sample in samples {
//         let (honest_blocks, adversary_blocks) = sample;

//         let scaled_honest_blocks = scale_blocks(&honest_blocks, g, 0.);
//         let scaled_adversary_blocks = scale_blocks(&adversary_blocks, g * beta / (1. - beta), 0.);

//         let honest_work_progresses = get_work_progresses::<HONEST_BLOCK_HEIGHT, HONEST_MAX_PROGRESS_HEIGHT>(scaled_honest_blocks, 1.);
//         let adversary_work_progresses = get_work_progresses::<ADVERSARY_BLOCK_HEIGHT, ADVERSARY_MAX_PROGRESS_HEIGHT>(scaled_adversary_blocks, 0.);

//         let k = get_k(&honest_work_progresses, &adversary_work_progresses);
//         let f = honest_work_progresses.last().unwrap().work / adversary_work_progresses.last().unwrap().timestamp;
//         println!("f: {}", f);
//         println!("k: {}", k);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_work_progresses() {
        let blocks = [
            Block {
                timestamp: 0.21609444842359038,
                work: 1.0,
            },
            Block {
                timestamp: 0.9941251768408977,
                work: 1.0,
            },
            Block {
                timestamp: 1.2046452169979325,
                work: 1.0,
            },
            Block {
                timestamp: 2.136048217394383,
                work: 1.0,
            },
            Block {
                timestamp: 3.423213223847526,
                work: 1.0,
            },
            Block {
                timestamp: 3.6189603860223345,
                work: 1.0,
            },
            Block {
                timestamp: 8.792463993295335,
                work: 1.0,
            },
            Block {
                timestamp: 9.795002516700476,
                work: 1.0,
            },
            Block {
                timestamp: 10.36318196782859,
                work: 1.0,
            },
            Block {
                timestamp: 10.560849890416328,
                work: 1.0,
            },
        ];

        assert_eq!(
            get_work_progresses::<10, 11>(blocks.clone(), 0.),
            vec![
                Chain {
                    timestamp: 0.,
                    work: 0.,
                    height: 0,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                },
                Chain {
                    timestamp: 0.9941251768408977,
                    work: 2.0,
                    height: 2,
                },
                Chain {
                    timestamp: 1.2046452169979325,
                    work: 3.0,
                    height: 3,
                },
                Chain {
                    timestamp: 2.136048217394383,
                    work: 4.0,
                    height: 4,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 5.0,
                    height: 5,
                },
                Chain {
                    timestamp: 3.6189603860223345,
                    work: 6.0,
                    height: 6,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 7.0,
                    height: 7,
                },
                Chain {
                    timestamp: 9.795002516700476,
                    work: 8.0,
                    height: 8,
                },
                Chain {
                    timestamp: 10.36318196782859,
                    work: 9.0,
                    height: 9,
                },
                Chain {
                    timestamp: 10.560849890416328,
                    work: 10.0,
                    height: 10,
                },
            ]
        );

        assert_eq!(
            get_work_progresses::<10, 11>(blocks.clone(), 0.5),
            vec![
                Chain {
                    timestamp: 0.,
                    work: 0.,
                    height: 0,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                },
                Chain {
                    timestamp: 0.9941251768408977,
                    work: 2.0,
                    height: 2,
                },
                Chain {
                    timestamp: 2.136048217394383,
                    work: 3.0,
                    height: 3,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 4.0,
                    height: 4,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 5.0,
                    height: 5,
                },
                Chain {
                    timestamp: 9.795002516700476,
                    work: 6.0,
                    height: 6,
                },
                Chain {
                    timestamp: 10.36318196782859,
                    work: 7.0,
                    height: 7,
                }
            ]
        );

        assert_eq!(
            get_work_progresses::<10, 11>(blocks.clone(), 1.),
            vec![
                Chain {
                    timestamp: 0.0,
                    work: 0.0,
                    height: 0
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1
                },
                Chain {
                    timestamp: 2.136048217394383,
                    work: 2.0,
                    height: 2
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 3.0,
                    height: 3
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 4.0,
                    height: 4
                },
                Chain {
                    timestamp: 9.795002516700476,
                    work: 5.0,
                    height: 5
                }
            ]
        );

        assert_eq!(
            get_work_progresses::<10, 11>(blocks.clone(), 2.),
            vec![
                Chain {
                    timestamp: 0.,
                    work: 0.,
                    height: 0,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 2.0,
                    height: 2,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 3.0,
                    height: 3,
                },
            ]
        );
    }

    #[test]
    fn test_get_k() {
        // Adversary gets in-front at one point but then honest recover

        let honest_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
            },
            Chain {
                timestamp: 3.0,
                work: 4.0,
                height: 2,
            },
            Chain {
                timestamp: 4.0,
                work: 6.0,
                height: 3,
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 2.0,
                work: 3.0,
                height: 1,
            },
            Chain {
                timestamp: 5.0,
                work: 4.0,
                height: 2,
            },
            Chain {
                timestamp: 6.0,
                work: 5.0,
                height: 3,
            },
        ];

        assert_eq!(get_k(&honest_sample, &adversary_sample), 4.0);

        // Adversary gets in-front for a while but then honest recovers
        let honest_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
            },
            Chain {
                timestamp: 4.0,
                work: 3.0,
                height: 2,
            },
            Chain {
                timestamp: 5.0,
                work: 6.0,
                height: 3,
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 2.0,
                work: 3.0,
                height: 1,
            },
            Chain {
                timestamp: 3.0,
                work: 4.0,
                height: 2,
            },
            Chain {
                timestamp: 6.0,
                work: 5.0,
                height: 3,
            },
        ];

        assert_eq!(get_k(&honest_sample, &adversary_sample), 6.0);

        // Adversary gets in-front and stays in-front
        let honest_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 2.0,
                work: 1.0,
                height: 1,
            },
            Chain {
                timestamp: 3.0,
                work: 2.0,
                height: 2,
            },
            Chain {
                timestamp: 5.0,
                work: 4.0,
                height: 3,
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 2.0,
                work: 2.0,
                height: 1,
            },
            Chain {
                timestamp: 3.0,
                work: 3.0,
                height: 2,
            },
            Chain {
                timestamp: 4.0,
                work: 4.0,
                height: 3,
            },
        ];

        assert_eq!(get_k(&honest_sample, &adversary_sample), INF);

        // Honest are in-front at first but then adversary surpasses them
        let honest_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
            },
            Chain {
                timestamp: 4.0,
                work: 3.0,
                height: 2,
            },
            Chain {
                timestamp: 6.0,
                work: 4.0,
                height: 3,
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 2.0,
                work: 1.0,
                height: 1,
            },
            Chain {
                timestamp: 3.0,
                work: 3.0,
                height: 2,
            },
            Chain {
                timestamp: 5.0,
                work: 5.0,
                height: 3,
            },
        ];

        assert_eq!(get_k(&honest_sample, &adversary_sample), INF);

        // Adversary gets equal work to honest at some point but honest surpasses them after, check off-by-one for equality
        let honest_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
            },
            Chain {
                timestamp: 3.0,
                work: 4.0,
                height: 2,
            },
            Chain {
                timestamp: 5.0,
                work: 6.0,
                height: 3,
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 0.0,
                work: 0.0,
                height: 0,
            },
            Chain {
                timestamp: 2.0,
                work: 3.0,
                height: 1,
            },
            Chain {
                timestamp: 4.0,
                work: 4.0,
                height: 2,
            },
            Chain {
                timestamp: 6.0,
                work: 5.0,
                height: 3,
            },
        ];

        assert_eq!(get_k(&honest_sample, &adversary_sample), 6.0);
    }
}
