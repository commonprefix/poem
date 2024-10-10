use crate::{
    analysis::get_monte_carlo_performance,
    formatting::{
        get_monte_carlo_progresses, scale_monte_carlo_blocks, scale_monte_carlo_progresses,
    },
    sampling::{get_monte_carlo_bitcoin_executions, sample_monte_carlo_poem_executions},
    types::INF,
};

pub const HONEST_COUNT: usize = 1300;
pub const ADVERSARY_COUNT: usize = 1300;

pub struct Data {
    pub bitcoin_latencies: Vec<f64>,
    pub bitcoin_optimal_k: Vec<f64>,
    pub bitcoin_optimal_g: Vec<f64>,
    pub bitcoin_throughputs: Vec<f64>,
    pub poem_latencies: Vec<f64>,
    pub poem_optimal_k: Vec<f64>,
    pub poem_optimal_g: Vec<f64>,
    pub poem_optimal_gamma: Vec<f64>,
    pub poem_throughputs: Vec<f64>,
}

pub struct PoemData {
    pub latency: Vec<f64>,
    pub optimal_k: Vec<f64>,
    pub optimal_g: Vec<f64>,
    pub optimal_gamma: Vec<f64>,
    pub throughput: Vec<f64>,
    pub max_work: Vec<f64>,
    pub max_height: Vec<f64>,
    pub adversary_max_work: Vec<f64>,
    pub adversary_max_height: Vec<f64>,
}

pub struct BitcoinData {
    pub latency: Vec<f64>,
    pub optimal_k: Vec<f64>,
    pub optimal_g: Vec<f64>,
    pub throughput: Vec<f64>,
    pub max_work: Vec<f64>,
    pub max_height: Vec<f64>,
    pub adversary_max_work: Vec<f64>,
    pub adversary_max_height: Vec<f64>,
}

pub fn simulate_poem(
    timestamps: (Vec<[f64; HONEST_COUNT]>, Vec<[f64; ADVERSARY_COUNT]>),
    monte_carlo: usize,
    epsilon: f64,
    beta_range: Vec<f64>,
    g_range: Vec<f64>,
    gamma_range: Vec<f64>,
) -> PoemData {
    println!("Working on PoEM...");
    let mut poem_data = PoemData {
        latency: vec![INF; beta_range.len()],
        optimal_k: vec![INF; beta_range.len()],
        optimal_g: vec![0.0; beta_range.len()],
        optimal_gamma: vec![0.0; beta_range.len()],
        throughput: vec![0.0; beta_range.len()],
        max_work: vec![0.0; beta_range.len()],
        max_height: vec![0.0; beta_range.len()],
        adversary_max_work: vec![0.0; beta_range.len()],
        adversary_max_height: vec![0.0; beta_range.len()],
    };

    // Get the block creations
    let (poem_honest_monte_carlo, poem_adversary_monte_carlo) =
        sample_monte_carlo_poem_executions::<HONEST_COUNT, ADVERSARY_COUNT>(&timestamps);

    // Create the scaled honest blocks initial vector
    let mut scaled_poem_honest_monte_carlo = poem_honest_monte_carlo.clone();
    // Create the honest progress empty vector
    let mut poem_honest_progress_monte_carlo =
        vec![Vec::with_capacity(HONEST_COUNT + 1); monte_carlo];

    // Create the adversary progress empty vector
    let mut original_poem_adversary_progress_monte_carlo =
        vec![Vec::with_capacity(ADVERSARY_COUNT + 1); monte_carlo];
    // Initialize the adversary progress initial vector
    get_monte_carlo_progresses(
        &mut original_poem_adversary_progress_monte_carlo,
        &poem_adversary_monte_carlo,
        0.,
    );
    // Create the scaled adversary progress empty vector
    let mut scaled_poem_adversary_progress_monte_carlo =
        original_poem_adversary_progress_monte_carlo.clone();

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
                let (
                    k,
                    f_work,
                    f_height,
                    max_work,
                    max_height,
                    adversary_max_work,
                    adversary_max_height,
                ) = get_monte_carlo_performance(
                    &poem_honest_progress_monte_carlo,
                    &scaled_poem_adversary_progress_monte_carlo,
                    epsilon,
                );
                let poem_latency = k / f_work;
                if poem_latency < poem_data.latency[beta_index] {
                    poem_data.latency[beta_index] = poem_latency;
                    poem_data.optimal_k[beta_index] = k;
                    poem_data.optimal_gamma[beta_index] = gamma;
                    poem_data.optimal_g[beta_index] = g;
                    poem_data.throughput[beta_index] = f_height;
                    poem_data.max_work[beta_index] = max_work;
                    poem_data.max_height[beta_index] = max_height;
                    poem_data.adversary_max_work[beta_index] = adversary_max_work;
                    poem_data.adversary_max_height[beta_index] = adversary_max_height;
                }
            }
        }
    }
    poem_data
}

pub fn simulate_bitcoin(
    timestamps: (Vec<[f64; HONEST_COUNT]>, Vec<[f64; ADVERSARY_COUNT]>),
    monte_carlo: usize,
    epsilon: f64,
    beta_range: Vec<f64>,
    g_range: Vec<f64>,
) -> BitcoinData {
    println!("Working on Bitcoin...");
    let mut bitcoin_data = BitcoinData {
        latency: vec![INF; beta_range.len()],
        optimal_k: vec![INF; beta_range.len()],
        optimal_g: vec![0.0; beta_range.len()],
        throughput: vec![0.0; beta_range.len()],
        max_work: vec![0.0; beta_range.len()],
        max_height: vec![0.0; beta_range.len()],
        adversary_max_work: vec![0.0; beta_range.len()],
        adversary_max_height: vec![0.0; beta_range.len()],
    };

    let (bitcoin_honest_monte_carlo, bitcoin_adversary_monte_carlo) =
        get_monte_carlo_bitcoin_executions::<HONEST_COUNT, ADVERSARY_COUNT>(&timestamps);

    // Create the scaled honest blocks initial vector
    let mut scaled_bitcoin_honest_monte_carlo = bitcoin_honest_monte_carlo.clone();
    // Create the honest progress empty vector
    let mut bitcoin_honest_progress_monte_carlo =
        vec![Vec::with_capacity(HONEST_COUNT + 1); monte_carlo];

    // Create the adversary progress empty vector
    let mut original_bitcoin_adversary_progress_monte_carlo =
        vec![Vec::with_capacity(ADVERSARY_COUNT + 1); monte_carlo];
    // Initialize the adversary progress initial vector
    get_monte_carlo_progresses(
        &mut original_bitcoin_adversary_progress_monte_carlo,
        &bitcoin_adversary_monte_carlo,
        0.,
    );
    // Create the scaled adversary progress empty vector
    let mut scaled_bitcoin_adversary_progress_monte_carlo =
        original_bitcoin_adversary_progress_monte_carlo.clone();

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

            let (
                k,
                f_work,
                f_height,
                max_work,
                max_height,
                adversary_max_work,
                adversary_max_height,
            ) = get_monte_carlo_performance(
                &bitcoin_honest_progress_monte_carlo,
                &scaled_bitcoin_adversary_progress_monte_carlo,
                epsilon,
            );
            let bitcoin_latency = k / f_work;
            if bitcoin_latency < bitcoin_data.latency[beta_index] {
                bitcoin_data.latency[beta_index] = bitcoin_latency;
                bitcoin_data.optimal_k[beta_index] = k;
                bitcoin_data.optimal_g[beta_index] = g;
                bitcoin_data.throughput[beta_index] = f_height;
                bitcoin_data.max_work[beta_index] = max_work;
                bitcoin_data.max_height[beta_index] = max_height;
                bitcoin_data.adversary_max_work[beta_index] = adversary_max_work;
                bitcoin_data.adversary_max_height[beta_index] = adversary_max_height;
            }
        }
    }
    bitcoin_data
}

// pub fn poem_throughput_at_optimal_latency(
//     monte_carlo: usize,
//     epsilon: f64,
//     beta_range: Vec<f64>,
//     optimal_g_range: Vec<f64>,
//     optimal_gamma_range: Vec<f64>,
// ) -> Vec<f64> {
//     let mut poem_throughputs = vec![];
//     for (beta, (g, gamma)) in zip(beta_range, zip(optimal_g_range, optimal_gamma_range)) {
//         let data = poem_vs_bitcoin(
//             monte_carlo, epsilon, vec![beta], vec![g], vec![gamma]);
//         poem_throughputs.push(data.poem_throughputs[0]);
//     }
//     poem_throughputs
// }

// pub fn bitcoin_throughput_at_optimal_latency(
//     monte_carlo: usize,
//     epsilon: f64,
//     beta_range: Vec<f64>,
//     optimal_g_range: Vec<f64>,
// ) -> Vec<f64> {
//     let mut bitcoin_throughputs = vec![];
//     for (beta, g) in zip(beta_range, optimal_g_range) {
//         let data = poem_vs_bitcoin(
//             monte_carlo, epsilon, vec![beta], vec![g], vec![0.0]);
//         bitcoin_throughputs.push(data.bitcoin_throughputs[0]);
//     }
//     bitcoin_throughputs
// }
