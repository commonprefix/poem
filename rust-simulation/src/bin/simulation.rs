use std::{fs::File, io::Write};

use clap::{ArgGroup, Parser};
use rust_simulation::simulations::{
    get_optimal_bitcoin_latency, get_optimal_poem_parameters,
    poem_latency_per_gamma_fixed_g,
};
use serde_json::json;
/// Custom error type for range parsing
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
        .args(&["gamma_latency", "bitcoin_latency", "poem_latency"])
        .required(true)
        .multiple(false)
))]
struct Args {
    #[arg(long)]
    poem_latency: bool,
    #[arg(long)]
    bitcoin_latency: bool,
    #[arg(long)]
    gamma_latency: bool,
    #[arg(long)]
    gamma_range: Option<String>,
    #[arg(long)]
    beta_range: Option<String>,
    #[arg(long)]
    g_range: Option<String>,
    #[arg(long)]
    beta: Option<f64>,
    #[arg(short)]
    g: Option<f64>,
    #[arg(long, default_value = "1000")]
    monte_carlo: usize,
    #[arg(long, default_value = "0.1")]
    error: f64,
    #[arg(long, default_value = "0.05")]
    variance_margin: f64,
}

fn parse_range(s: String) -> Result<Vec<f64>, RangeParseError> {
    let formatted = s.replace("..", " ").replace(":", " ");
    let numbers: Vec<f64> = formatted
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok()) // Parse each segment into f64
        .collect();

    let mut range: Vec<f64> = vec![];

    if numbers.len() == 3 {
        let start = numbers[0];
        let end = numbers[1];
        let step = numbers[2];

        let mut current = start;
        while current <= end {
            range.push(current);
            current += step;
        }

        Ok(range)
    } else {
        Err(RangeParseError)
    }
}

fn main() {
    let args = Args::parse();
    println!(
        "Simulating with MONTE_CARLO = {} and error = {}",
        args.monte_carlo, args.error
    );

    if args.poem_latency {
        let beta_range = parse_range(args.beta_range.clone().unwrap()).unwrap();
        let g_range = parse_range(args.g_range.clone().unwrap()).unwrap();
        let gamma_range = parse_range(args.gamma_range.clone().unwrap()).unwrap();

        println!("Finding optimal PoEM latency for various β");
        let file_name = format!(
            "simulation_data/poem_latency_monte_carlo_{}.json",
            args.monte_carlo
        );
        let mut file = File::create(file_name.clone()).unwrap();

        let (optimal_g, optimal_gamma, optimal_latency, throughput) = get_optimal_poem_parameters(
            args.monte_carlo as i32,
            args.error,
            g_range,
            beta_range.clone(),
            gamma_range,
        );

        println!("beta: {:?}", beta_range);
        println!("Latency: {:?}", optimal_latency.clone());

        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "latency": optimal_latency,
            "optimal_g": optimal_g,
            "optimal_gamma": optimal_gamma,
            "throughput": throughput,
        });
        let json_string = serde_json::to_string_pretty(&data).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
        println!("Wrote to file: {}", file_name);
    }

    if args.bitcoin_latency {
        let beta_range = parse_range(args.beta_range.clone().unwrap()).unwrap();
        let g_range = parse_range(args.g_range.clone().unwrap()).unwrap();

        println!("Finding optimal Bitcoin latency for various β");
        let file_name = format!(
            "simulation_data/bitcoin_latency_beta_{}_monte_carlo_{}.json",
            args.beta_range.unwrap(),
            args.monte_carlo,
        );
        let mut file = File::create(file_name.clone()).unwrap();

        let latency_per_beta = get_optimal_bitcoin_latency(
            args.monte_carlo as i32,
            args.error,
            g_range,
            beta_range.clone(),
        );

        println!("beta: {:?}", beta_range);
        println!("Latency: {:?}", latency_per_beta.clone());

        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "latency": latency_per_beta,
        });
        let json_string = serde_json::to_string_pretty(&data).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
        println!("Wrote to file: {}", file_name);
    }

    if args.gamma_latency {
        if args.beta.is_some() && args.g.is_some() {
            let gamma_range = parse_range(args.gamma_range.clone().unwrap()).unwrap();
            println!(
                "Finding latency for various γ (fixed β = {} and g = {})",
                args.beta.unwrap(),
                args.g.unwrap()
            );
            let file_name = format!(
                "simulation_data/gamma_latency_β_{}_g_{}_gamma_{}_monte_carlo_{}.json",
                args.beta.unwrap(),
                args.g.unwrap(),
                args.gamma_range.unwrap(),
                args.monte_carlo
            );
            let mut file = File::create(file_name.clone()).unwrap();

            let latency_per_gamma = poem_latency_per_gamma_fixed_g(
                args.monte_carlo as i32,
                args.error,
                args.g.unwrap(),
                vec![args.beta.unwrap()],
                gamma_range.clone(),
            );

            let latency: Vec<f64> = latency_per_gamma.iter().map(|x| x[0]).collect();

            println!("γ: {:?}", gamma_range);
            println!("Latency: {:?}", latency.clone());

            let data = json!({
                "monte_carlo": args.monte_carlo,
                "error": args.error,
                "beta": args.beta.unwrap(),
                "g": args.g.unwrap(),
                "gamma": gamma_range,
                "latency": latency,
            });
            let json_string = serde_json::to_string_pretty(&data).unwrap();

            file.write_all(json_string.as_bytes()).unwrap();
            println!("Wrote to file: {}", file_name);
        }
    }
}
