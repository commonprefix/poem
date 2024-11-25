use std::{fs::File, io::Write};

use clap::{ArgGroup, Parser};
use serde_json::json;
use simulation::{
    sampling::sample_monte_carlo_execution_timestamps,
    simulations::{simulate_bitcoin, simulate_poem, ReductionType, ADVERSARY_COUNT, HONEST_COUNT},
};

#[derive(Debug)]
struct RangeParseError;

impl std::fmt::Display for RangeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid range format. Use the format: start..end:step")
    }
}

impl std::error::Error for RangeParseError {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("commands")
        .args(&["bitcoin_vs_poem", "poem", "g_latency", "gamma_latency"])
        .required(true)
        .multiple(false)
))]
struct Args {
    #[arg(long)]
    gamma_latency: bool,
    #[arg(long)]
    g_latency: bool,
    #[arg(long)]
    bitcoin_vs_poem: bool,
    #[arg(long)]
    poem: bool,
    #[arg(long)]
    gamma_range: Option<String>,
    #[arg(long)]
    gamma: Option<f64>,
    #[arg(long)]
    g_range: Option<String>,
    #[arg(long)]
    g: Option<f64>,
    #[arg(long)]
    beta_range: Option<String>,
    #[arg(long)]
    beta: Option<f64>,
    #[arg(long, default_value = "1000")]
    monte_carlo: usize,
    #[arg(long, default_value = "0.1")]
    error: f64,
}

fn logarithmic_range(start: f64, end: f64, num_points: usize, exponent: f64) -> Vec<f64> {
    (0..num_points)
        .map(|i| {
            let fraction = (i as f64 / (num_points - 1) as f64).powf(exponent);
            start * (end / start).powf(fraction)
        })
        .collect::<Vec<f64>>()
}

fn linear_range(start: f64, end: f64, num_points: usize) -> Vec<f64> {
    (0..num_points)
        .map(|i| start + (end - start) * (i as f64 / (num_points - 1) as f64))
        .collect::<Vec<f64>>()
}

fn parse_range(s: String, exponent: Option<f64>) -> Result<Vec<f64>, RangeParseError> {
    let parts: Vec<&str> = s.split(':').collect();

    if parts.len() != 3 {
        return Err(RangeParseError);
    }
    let start: f64 = parts[0].parse().expect("Failed to parse start");
    let end: f64 = parts[1].parse().expect("Failed to parse end");
    let num_points: usize = parts[2].parse().expect("Failed to parse num_points");

    if let Some(exp) = exponent {
        if start == 0.0 {
            let mut range = logarithmic_range(start + 0.005, end, num_points, exp);
            if range.len() > 1 {
                range.remove(0);
            }
            range.insert(0, 0.0);
            return Ok(range);
        }
        Ok(logarithmic_range(start, end, num_points, exp))
    } else {
        Ok(linear_range(start, end, num_points))
    }
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();

    if args.gamma_latency {
        let gamma_range = parse_range(args.gamma_range.clone().unwrap(), Some(0.5)).unwrap();
        let beta_range = vec![args.beta.clone().unwrap()];
        let g_range = vec![args.g.clone().unwrap()];

        println!("G range: {:?}", g_range);
        println!("Beta range: {:?}", beta_range);
        println!("Gamma range: {:?}", gamma_range);

        let timestamps = sample_monte_carlo_execution_timestamps::<HONEST_COUNT, ADVERSARY_COUNT>(
            args.monte_carlo,
        );
        let poem_data = simulate_poem(
            timestamps.clone(),
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
            gamma_range.clone(),
            ReductionType::Gamma,
        );

        let bitcoin_data = simulate_bitcoin(
            timestamps,
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
        );

        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "g": g_range,
            "gamma": gamma_range,
            "poem_latency": poem_data.latency,
            "bitcoin_latency": bitcoin_data.latency,
        });
        let json_string = serde_json::to_string_pretty(&data).unwrap();
        let file_name = format!(
            "simulation_data/poem_gamma_latency_beta_{}_g_{}_gamma_{}_monte_carlo_{}_error_{}.json",
            args.beta.clone().unwrap(),
            args.g.clone().unwrap(),
            args.gamma_range.clone().unwrap(),
            args.monte_carlo,
            args.error
        );
        let mut file = File::create(file_name.clone()).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
        println!("Wrote to file: {}", file_name);
    }

    if args.g_latency {
        let g_range = parse_range(args.g_range.clone().unwrap(), None).unwrap();
        let beta_range = vec![args.beta.clone().unwrap()];
        let gamma_range = vec![args.gamma.clone().unwrap()];

        println!("G range: {:?}", g_range);
        println!("Beta: {:?}", beta_range);
        println!("Gamma: {:?}", gamma_range);

        let timestamps = sample_monte_carlo_execution_timestamps::<HONEST_COUNT, ADVERSARY_COUNT>(
            args.monte_carlo,
        );
        let poem_data = simulate_poem(
            timestamps,
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
            gamma_range.clone(),
            ReductionType::G,
        );

        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "g": g_range,
            "gamma": gamma_range,
            "latency": poem_data.latency,
            "optimal_k": poem_data.optimal_k,
            "optimal_g": poem_data.optimal_g,
            "optimal_gamma": poem_data.optimal_gamma,
            "throughput": poem_data.throughput,
            "max_work": poem_data.max_work,
            "max_height": poem_data.max_height,
            "adversary_max_work": poem_data.adversary_max_work,
            "adversary_max_height": poem_data.adversary_max_height,
        });
        let json_string = serde_json::to_string_pretty(&data).unwrap();
        let file_name = format!(
            "simulation_data/poem_g_latency_beta_{}_g_{}_gamma_{}_monte_carlo_{}_error_{}.json",
            args.beta.clone().unwrap(),
            args.g_range.clone().unwrap(),
            args.gamma.clone().unwrap(),
            args.monte_carlo,
            args.error
        );
        let mut file = File::create(file_name.clone()).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
        println!("Wrote to file: {}", file_name);
    }

    if args.poem {
        let beta_range = parse_range(args.beta_range.clone().unwrap(), None).unwrap();
        let g_range = parse_range(args.g_range.clone().unwrap(), Some(0.5)).unwrap();
        let gamma_range = parse_range(args.gamma_range.clone().unwrap(), Some(0.5)).unwrap();
        println!("Beta range: {:?}", beta_range);
        println!("G range: {:?}", g_range);
        println!("Gamma range: {:?}", gamma_range);

        let timestamps = sample_monte_carlo_execution_timestamps::<HONEST_COUNT, ADVERSARY_COUNT>(
            args.monte_carlo,
        );
        let poem_data = simulate_poem(
            timestamps,
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
            gamma_range.clone(),
            ReductionType::Beta,
        );

        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "g": g_range,
            "gamma": gamma_range,
            "latency": poem_data.latency,
            "optimal_k": poem_data.optimal_k,
            "optimal_g": poem_data.optimal_g,
            "optimal_gamma": poem_data.optimal_gamma,
            "throughput": poem_data.throughput,
            "max_work": poem_data.max_work,
            "max_height": poem_data.max_height,
            "adversary_max_work": poem_data.adversary_max_work,
            "adversary_max_height": poem_data.adversary_max_height,
        });
        let json_string = serde_json::to_string_pretty(&data).unwrap();
        let file_name = format!(
            "simulation_data/poem_beta_{}_g_{}_gamma_{}_monte_carlo_{}_error_{}.json",
            args.beta_range.clone().unwrap(),
            args.g_range.clone().unwrap(),
            args.gamma_range.clone().unwrap(),
            args.monte_carlo,
            args.error
        );
        let mut file = File::create(file_name.clone()).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
        println!("Wrote to file: {}", file_name);
    }

    if args.bitcoin_vs_poem {
        let beta_range = parse_range(args.beta_range.clone().unwrap(), None).unwrap();
        let g_range = parse_range(args.g_range.clone().unwrap(), Some(0.5)).unwrap();
        let gamma_range = parse_range(args.gamma_range.clone().unwrap(), Some(0.5)).unwrap();
        println!("Beta range: {:?}", beta_range);
        println!("G range: {:?}", g_range);
        println!("Gamma range: {:?}", gamma_range);

        let timestamps = sample_monte_carlo_execution_timestamps::<HONEST_COUNT, ADVERSARY_COUNT>(
            args.monte_carlo,
        );
        let poem_data = simulate_poem(
            timestamps.clone(),
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
            gamma_range.clone(),
            ReductionType::Beta,
        );

        let bitcoin_data = simulate_bitcoin(
            timestamps,
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
        );

        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "g": g_range,
            "gamma": gamma_range,

            "bitcoin_latency": bitcoin_data.latency,
            "bitcoin_optimal_k": bitcoin_data.optimal_k,
            "bitcoin_optimal_g": bitcoin_data.optimal_g,
            "bitcoin_throughput": bitcoin_data.throughput,
            "bitcoin_max_work": bitcoin_data.max_work,
            "bitcoin_max_height": bitcoin_data.max_height,
            "bitcoin_adversary_max_work": bitcoin_data.adversary_max_work,
            "bitcoin_adversary_max_height": bitcoin_data.adversary_max_height,

            "poem_latency": poem_data.latency,
            "poem_optimal_k": poem_data.optimal_k,
            "poem_optimal_g": poem_data.optimal_g,
            "poem_optimal_gamma": poem_data.optimal_gamma,
            "poem_throughput": poem_data.throughput,
            "poem_max_work": poem_data.max_work,
            "poem_max_height": poem_data.max_height,
            "poem_adversary_max_work": poem_data.adversary_max_work,
            "poem_adversary_max_height": poem_data.adversary_max_height,
        });
        let json_string = serde_json::to_string_pretty(&data).unwrap();
        let file_name = format!(
            "simulation_data/bitcoin_vs_poem_beta_{}_g_{}_gamma_{}_monte_carlo_{}_error_{}.json",
            args.beta_range.clone().unwrap(),
            args.g_range.clone().unwrap(),
            args.gamma_range.clone().unwrap(),
            args.monte_carlo,
            args.error
        );
        let mut file = File::create(file_name.clone()).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
        println!("Wrote to file: {}", file_name);
    }

    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}
