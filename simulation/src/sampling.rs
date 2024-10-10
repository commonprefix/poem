use rand_distr::{Distribution, Exp};
use rayon::prelude::*;

use crate::types::Block;

fn sample_timestamps<T: rand::Rng, const N: usize>(mut rng: &mut T) -> [f64; N] {
    let time_distribution = Exp::new(1.0).unwrap();
    let mut block_time = 0.0;
    let mut block_timestamps = [0.0; N];

    for i in 0..N {
        block_time += time_distribution.sample(&mut rng);
        block_timestamps[i] = block_time;
    }

    block_timestamps
}

fn sample_monte_carlo_timestamps<const N: usize>(monte_carlo: usize) -> Vec<[f64; N]> {
    (0..monte_carlo)
        // .into_iter()
        .into_par_iter()
        .map(|_| sample_timestamps(&mut rand::thread_rng()))
        .collect()
}

pub fn sample_monte_carlo_execution_timestamps<
    const HONEST_HEIGHT: usize,
    const ADVERSARY_HEIGHT: usize,
>(
    monte_carlo: usize,
) -> (Vec<[f64; HONEST_HEIGHT]>, Vec<[f64; ADVERSARY_HEIGHT]>) {
    let honest_timestamps = sample_monte_carlo_timestamps::<HONEST_HEIGHT>(monte_carlo);
    let adversary_timestamps = sample_monte_carlo_timestamps::<ADVERSARY_HEIGHT>(monte_carlo);
    (honest_timestamps, adversary_timestamps)
}

fn get_bitcoin_blocks<const N: usize>(block_timestamps: [f64; N]) -> [Block; N] {
    block_timestamps.map(|t| Block {
        timestamp: t,
        work: 1.,
    })
}

fn get_monte_carlo_bitcoin_blocks<const N: usize>(
    monte_carlo_timestamps: &Vec<[f64; N]>,
) -> Vec<[Block; N]> {
    monte_carlo_timestamps
        // .iter()
        .par_iter()
        .map(|&block_timestamps| get_bitcoin_blocks(block_timestamps))
        .collect()
}

pub fn get_monte_carlo_bitcoin_executions<
    const HONEST_HEIGHT: usize,
    const ADVERSARY_HEIGHT: usize,
>(
    (honest_timestamps, adversary_timestamps): &(
        Vec<[f64; HONEST_HEIGHT]>,
        Vec<[f64; ADVERSARY_HEIGHT]>,
    ),
) -> (Vec<[Block; HONEST_HEIGHT]>, Vec<[Block; ADVERSARY_HEIGHT]>) {
    let honest_bitcoin_blocks = get_monte_carlo_bitcoin_blocks::<HONEST_HEIGHT>(&honest_timestamps);
    let adversary_bitcoin_blocks =
        get_monte_carlo_bitcoin_blocks::<ADVERSARY_HEIGHT>(&adversary_timestamps);

    (honest_bitcoin_blocks, adversary_bitcoin_blocks)
}

fn sample_poem_blocks<T: rand::Rng, const N: usize>(
    block_timestamps: [f64; N],
    work_distribution: Exp<f64>,
    rng: &mut T,
) -> [Block; N] {
    block_timestamps.map(|t| Block {
        timestamp: t,
        work: work_distribution.sample(rng),
    })
}

fn sample_monte_carlo_poem_blocks<const N: usize>(
    monte_carlo_timestamps: &Vec<[f64; N]>,
) -> Vec<[Block; N]> {
    let work_distribution = Exp::new(std::f64::consts::LN_2).unwrap();
    monte_carlo_timestamps
        // .iter()
        .par_iter()
        .map(|&block_timestamps| {
            sample_poem_blocks(block_timestamps, work_distribution, &mut rand::thread_rng())
        })
        .collect()
}

pub fn sample_monte_carlo_poem_executions<
    const HONEST_HEIGHT: usize,
    const ADVERSARY_HEIGHT: usize,
>(
    (honest_timestamps, adversary_timestamps): &(
        Vec<[f64; HONEST_HEIGHT]>,
        Vec<[f64; ADVERSARY_HEIGHT]>,
    ),
) -> (Vec<[Block; HONEST_HEIGHT]>, Vec<[Block; ADVERSARY_HEIGHT]>) {
    let honest_poem_blocks = sample_monte_carlo_poem_blocks::<HONEST_HEIGHT>(&honest_timestamps);
    let adversary_poem_blocks =
        sample_monte_carlo_poem_blocks::<ADVERSARY_HEIGHT>(&adversary_timestamps);

    (honest_poem_blocks, adversary_poem_blocks)
}
