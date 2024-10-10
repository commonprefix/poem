use crate::types::{Block, Chain};
use rayon::prelude::*;

#[inline]
fn scale_blocks<const N: usize>(
    scaled_blocks: &mut [Block; N],
    original_blocks: &[Block; N],
    g: f64,
    gamma: f64,
) {
    scaled_blocks
        .iter_mut()
        .zip(original_blocks.iter())
        .for_each(|(scaled_block, original_block)| {
            scaled_block.timestamp = original_block.timestamp / g;
            scaled_block.work = original_block.work + gamma;
        });
}

#[inline]
pub fn scale_monte_carlo_blocks<const N: usize>(
    scaled_monte_carlo_blocks: &mut Vec<[Block; N]>,
    original_monte_carlo_blocks: &Vec<[Block; N]>,
    g: f64,
    gamma: f64,
) {
    scaled_monte_carlo_blocks
        .iter_mut()
        .zip(original_monte_carlo_blocks.iter())
        .for_each(|(scaled_blocks, original_blocks)| {
            scale_blocks(scaled_blocks, original_blocks, g, gamma)
        })
}

#[inline]
fn scale_progresses(
    scaled_progresses: &mut Vec<Chain>,
    original_progresses: &Vec<Chain>,
    g: f64,
    gamma: f64,
) {
    scaled_progresses
        .iter_mut()
        .zip(original_progresses.iter())
        .for_each(|(scaled_progress, original_progress)| {
            *scaled_progress = Chain {
                timestamp: original_progress.timestamp / g,
                work: original_progress.work + (original_progress.height as f64 * gamma),
                height: original_progress.height,
                arrival_time: 0.0,
            };
        });
}

#[inline]
pub fn scale_monte_carlo_progresses(
    scaled_monte_carlo_progresses: &mut Vec<Vec<Chain>>,
    original_monte_carlo_progresses: &Vec<Vec<Chain>>,
    g: f64,
    gamma: f64,
) {
    scaled_monte_carlo_progresses
        .iter_mut()
        .zip(original_monte_carlo_progresses.iter())
        .for_each(|(scaled_progresses, original_progresses)| {
            scale_progresses(scaled_progresses, original_progresses, g, gamma)
        })
}

#[inline]
fn get_progresses<const N: usize>(
    progresses: &mut Vec<Chain>,
    blocks: &[Block; N],
    network_delay: f64,
) {
    progresses.clear();
    progresses.push(Chain {
        timestamp: 0.0,
        work: 0.0,
        height: 0,
        arrival_time: 0.0,
    });

    let mut last_arrival_index = 0;

    for new_block in blocks {
        let mut last_arrival = &progresses[last_arrival_index];
        // Before processing the newly mined block first process all received blocks before it
        loop {
            if last_arrival_index + 1 >= progresses.len() {
                break;
            }
            let next_arrival = &progresses[last_arrival_index + 1];

            if next_arrival.arrival_time > new_block.timestamp {
                break;
            }

            last_arrival = next_arrival;
            last_arrival_index += 1;
        }

        // if the new block does not make new progress, skip it
        if last_arrival.work + new_block.work <= progresses.last().unwrap().work {
            continue;
        }

        progresses.push(Chain {
            timestamp: new_block.timestamp,
            work: last_arrival.work + new_block.work,
            height: last_arrival.height + 1,
            arrival_time: new_block.timestamp + network_delay,
        });
    }
}

#[inline]
pub fn get_monte_carlo_progresses<const N: usize>(
    monte_carlo_progresses: &mut Vec<Vec<Chain>>,
    monte_carlo_blocks: &Vec<[Block; N]>,
    network_delay: f64,
) {
    monte_carlo_progresses
        .par_iter_mut()
        .zip(monte_carlo_blocks.par_iter())
        .for_each(|(progresses, blocks)| {
            get_progresses(progresses, blocks, network_delay);
        });
}

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

        let mut progresses = Vec::with_capacity(blocks.len() + 1);

        get_progresses(&mut progresses, &blocks, 0.);
        assert_eq!(
            progresses,
            vec![
                Chain {
                    timestamp: 0.0,
                    work: 0.0,
                    height: 0,
                    arrival_time: progresses[0].arrival_time,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                    arrival_time: progresses[1].arrival_time,
                },
                Chain {
                    timestamp: 0.9941251768408977,
                    work: 2.0,
                    height: 2,
                    arrival_time: progresses[2].arrival_time,
                },
                Chain {
                    timestamp: 1.2046452169979325,
                    work: 3.0,
                    height: 3,
                    arrival_time: progresses[3].arrival_time,
                },
                Chain {
                    timestamp: 2.136048217394383,
                    work: 4.0,
                    height: 4,
                    arrival_time: progresses[4].arrival_time,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 5.0,
                    height: 5,
                    arrival_time: progresses[5].arrival_time,
                },
                Chain {
                    timestamp: 3.6189603860223345,
                    work: 6.0,
                    height: 6,
                    arrival_time: progresses[6].arrival_time,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 7.0,
                    height: 7,
                    arrival_time: progresses[7].arrival_time,
                },
                Chain {
                    timestamp: 9.795002516700476,
                    work: 8.0,
                    height: 8,
                    arrival_time: progresses[8].arrival_time,
                },
                Chain {
                    timestamp: 10.36318196782859,
                    work: 9.0,
                    height: 9,
                    arrival_time: progresses[9].arrival_time,
                },
                Chain {
                    timestamp: 10.560849890416328,
                    work: 10.0,
                    height: 10,
                    arrival_time: progresses[10].arrival_time,
                },
            ]
        );

        get_progresses(&mut progresses, &blocks, 0.5);
        assert_eq!(
            progresses,
            vec![
                Chain {
                    timestamp: 0.0,
                    work: 0.0,
                    height: 0,
                    arrival_time: progresses[0].arrival_time,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                    arrival_time: progresses[1].arrival_time,
                },
                Chain {
                    timestamp: 0.9941251768408977,
                    work: 2.0,
                    height: 2,
                    arrival_time: progresses[2].arrival_time,
                },
                Chain {
                    timestamp: 2.136048217394383,
                    work: 3.0,
                    height: 3,
                    arrival_time: progresses[3].arrival_time,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 4.0,
                    height: 4,
                    arrival_time: progresses[4].arrival_time,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 5.0,
                    height: 5,
                    arrival_time: progresses[5].arrival_time,
                },
                Chain {
                    timestamp: 9.795002516700476,
                    work: 6.0,
                    height: 6,
                    arrival_time: progresses[6].arrival_time,
                },
                Chain {
                    timestamp: 10.36318196782859,
                    work: 7.0,
                    height: 7,
                    arrival_time: progresses[7].arrival_time,
                },
            ]
        );
        get_progresses(&mut progresses, &blocks, 1.);
        assert_eq!(
            progresses,
            vec![
                Chain {
                    timestamp: 0.0,
                    work: 0.0,
                    height: 0,
                    arrival_time: progresses[0].arrival_time,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                    arrival_time: progresses[1].arrival_time,
                },
                Chain {
                    timestamp: 2.136048217394383,
                    work: 2.0,
                    height: 2,
                    arrival_time: progresses[2].arrival_time,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 3.0,
                    height: 3,
                    arrival_time: progresses[3].arrival_time,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 4.0,
                    height: 4,
                    arrival_time: progresses[4].arrival_time,
                },
                Chain {
                    timestamp: 9.795002516700476,
                    work: 5.0,
                    height: 5,
                    arrival_time: progresses[5].arrival_time,
                },
            ]
        );

        get_progresses(&mut progresses, &blocks, 2.);
        assert_eq!(
            progresses,
            vec![
                Chain {
                    timestamp: 0.0,
                    work: 0.0,
                    height: 0,
                    arrival_time: progresses[0].arrival_time,
                },
                Chain {
                    timestamp: 0.21609444842359038,
                    work: 1.0,
                    height: 1,
                    arrival_time: progresses[1].arrival_time,
                },
                Chain {
                    timestamp: 3.423213223847526,
                    work: 2.0,
                    height: 2,
                    arrival_time: progresses[2].arrival_time,
                },
                Chain {
                    timestamp: 8.792463993295335,
                    work: 3.0,
                    height: 3,
                    arrival_time: progresses[3].arrival_time,
                },
            ]
        );
    }
}
