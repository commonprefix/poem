use crate::{
    analysis::get_monte_carlo_performance,
    formatting::{
        get_monte_carlo_progresses, scale_monte_carlo_blocks, scale_monte_carlo_progresses,
    },
    sampling::{
        get_monte_carlo_bitcoin_executions, sample_monte_carlo_execution_timestamps,
        sample_monte_carlo_poem_executions,
    },
    types::INF,
};

const HONEST_HEIGHT: usize = 1800;
const ADVERSARY_HEIGHT: usize = 600;

pub fn poem_vs_bitcoin(
    monte_carlo: usize,
    epsilon: f64,
    beta_range: Vec<f64>,
    g_range: Vec<f64>,
    gamma_range: Vec<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let monte_carlo_execution_timestamps =
        sample_monte_carlo_execution_timestamps::<HONEST_HEIGHT, ADVERSARY_HEIGHT>(monte_carlo);

    // PoEM benchmark
    // Get the block creations
    let (poem_honest_monte_carlo, poem_adversary_monte_carlo) =
        sample_monte_carlo_poem_executions::<HONEST_HEIGHT, ADVERSARY_HEIGHT>(
            &monte_carlo_execution_timestamps,
        );

    // Create the scaled honest blocks initial vector
    let mut scaled_poem_honest_monte_carlo = poem_honest_monte_carlo.clone();
    // Create the honest progress empty vector
    let mut poem_honest_progress_monte_carlo =
        vec![Vec::with_capacity(HONEST_HEIGHT + 1); monte_carlo];

    // Create the adversary progress empty vector
    let mut original_poem_adversary_progress_monte_carlo =
        vec![Vec::with_capacity(ADVERSARY_HEIGHT + 1); monte_carlo];
    // Initialize the adversary progress initial vector
    get_monte_carlo_progresses(
        &mut original_poem_adversary_progress_monte_carlo,
        &poem_adversary_monte_carlo,
        0.,
    );
    // Create the scaled adversary progress empty vector
    let mut scaled_poem_adversary_progress_monte_carlo =
        original_poem_adversary_progress_monte_carlo.clone();

    let mut poem_latencies = vec![INF; beta_range.len()];
    let mut poem_optimal_gamma = vec![0.0; beta_range.len()];
    let mut poem_optimal_g = vec![0.0; beta_range.len()];

    println!("Working on PoEM...");
    for &gamma in gamma_range.iter() {
        println!("gamma: {}", gamma);
        for &g in g_range.iter() {
            println!(" - g: {}", g);
            // Scale PoEM honest blocks
            scale_monte_carlo_blocks(
                &mut scaled_poem_honest_monte_carlo,
                &poem_honest_monte_carlo,
                g,
                gamma,
            );

            // Get PoEM honest progress
            get_monte_carlo_progresses(
                &mut poem_honest_progress_monte_carlo,
                &scaled_poem_honest_monte_carlo,
                1.,
            );

            for (beta_index, &beta) in beta_range.iter().enumerate() {
                // Scale PoEM adversary progress
                scale_monte_carlo_progresses(
                    &mut scaled_poem_adversary_progress_monte_carlo,
                    &original_poem_adversary_progress_monte_carlo,
                    g * beta / (1. - beta),
                    gamma,
                );

                // Get configuration performance
                let (k, f_work, _) = get_monte_carlo_performance(
                    &poem_honest_progress_monte_carlo,
                    &scaled_poem_adversary_progress_monte_carlo,
                    epsilon,
                );
                let poem_latency = k / f_work;
                if poem_latency < poem_latencies[beta_index] {
                    poem_latencies[beta_index] = poem_latency;
                    poem_optimal_gamma[beta_index] = gamma;
                    poem_optimal_g[beta_index] = g;
                }
            }
        }
    }

    // Bitcoin benchmark
    let (bitcoin_honest_monte_carlo, bitcoin_adversary_monte_carlo) =
        get_monte_carlo_bitcoin_executions::<HONEST_HEIGHT, ADVERSARY_HEIGHT>(
            &monte_carlo_execution_timestamps,
        );

    // Create the scaled honest blocks initial vector
    let mut scaled_bitcoin_honest_monte_carlo = bitcoin_honest_monte_carlo.clone();
    // Create the honest progress empty vector
    let mut bitcoin_honest_progress_monte_carlo =
        vec![Vec::with_capacity(HONEST_HEIGHT + 1); monte_carlo];

    // Create the adversary progress empty vector
    let mut original_bitcoin_adversary_progress_monte_carlo =
        vec![Vec::with_capacity(ADVERSARY_HEIGHT + 1); monte_carlo];
    // Initialize the adversary progress initial vector
    get_monte_carlo_progresses(
        &mut original_bitcoin_adversary_progress_monte_carlo,
        &bitcoin_adversary_monte_carlo,
        0.,
    );
    // Create the scaled adversary progress empty vector
    let mut scaled_bitcoin_adversary_progress_monte_carlo =
        original_bitcoin_adversary_progress_monte_carlo.clone();

    let mut bitcoin_latencies = vec![INF; beta_range.len()];
    let mut bitcoin_optimal_g = vec![0.0; beta_range.len()];

    println!("Working on Bitcoin...");
    for &g in g_range.iter() {
        println!("g: {}", g);
        // Scale Bitcoin honest blocks
        scale_monte_carlo_blocks(
            &mut scaled_bitcoin_honest_monte_carlo,
            &bitcoin_honest_monte_carlo,
            g,
            0.0,
        );

        // Get Bitcoin honest progress
        get_monte_carlo_progresses(
            &mut bitcoin_honest_progress_monte_carlo,
            &scaled_bitcoin_honest_monte_carlo,
            1.,
        );

        for (beta_index, &beta) in beta_range.iter().enumerate() {
            // Scale Bitcoin adversary progress
            scale_monte_carlo_progresses(
                &mut scaled_bitcoin_adversary_progress_monte_carlo,
                &original_bitcoin_adversary_progress_monte_carlo,
                g * beta / (1. - beta),
                0.0,
            );

            let (k, f_work, _) = get_monte_carlo_performance(
                &bitcoin_honest_progress_monte_carlo,
                &scaled_bitcoin_adversary_progress_monte_carlo,
                epsilon,
            );
            let bitcoin_latency = k / f_work;
            if bitcoin_latency < bitcoin_latencies[beta_index] {
                bitcoin_latencies[beta_index] = bitcoin_latency;
                bitcoin_optimal_g[beta_index] = g;
            }
        }
    }

    (
        bitcoin_latencies,
        bitcoin_optimal_g,
        poem_latencies,
        poem_optimal_g,
        poem_optimal_gamma,
    )
}
