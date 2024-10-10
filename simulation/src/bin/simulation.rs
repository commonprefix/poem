use std::{fs::File, io::Write};

use clap::{ArgGroup, Parser};
use simulation::simulations::poem_vs_bitcoin;
use serde_json::json;


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
        .args(&["bitcoin_vs_poem"])
        .required(true)
        .multiple(false)
))]
struct Args {
    #[arg(long)]
    bitcoin_vs_poem: bool,
    #[arg(long)]
    gamma_range: Option<String>,
    #[arg(long)]
    g_range: Option<String>,
    #[arg(long)]
    beta_range: Option<String>,
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
    .map(|i| {
        start + (end - start) * (i as f64 / (num_points - 1) as f64)
    })
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
            return Ok(vec![0.0].into_iter().chain(logarithmic_range(start + 0.005, end, num_points, exp)[1..].to_vec()).collect());
        }
        Ok(logarithmic_range(start, end, num_points, exp))
    } else {
        Ok(linear_range(start, end, num_points))
    }
}


fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();

    if args.bitcoin_vs_poem {
        let beta_range = parse_range(args.beta_range.clone().unwrap(), None).unwrap();
        let g_range = parse_range(args.g_range.clone().unwrap(), Some(0.5)).unwrap();
        let gamma_range = parse_range(args.gamma_range.clone().unwrap(), Some(0.5)).unwrap();
        println!("Beta range: {:?}", beta_range);
        println!("G range: {:?}", g_range);
        println!("Gamma range: {:?}", gamma_range);

        let (
            bitcoin_latencies,
            bitcoin_optimal_g,
            bitcoin_throughputs,
            poem_latencies,
            poem_optimal_g,
            poem_optimal_gamma,
            poem_throughputs,
        ) = poem_vs_bitcoin(
            args.monte_carlo,
            args.error,
            beta_range.clone(),
            g_range.clone(),
            gamma_range.clone(),
        );
        let data = json!({
            "monte_carlo": args.monte_carlo,
            "error": args.error,
            "beta": beta_range,
            "g": g_range,
            "gamma": gamma_range,
            "bitcoin_latency": bitcoin_latencies,
            "bitcoin_optimal_g": bitcoin_optimal_g,
            "bitcoin_throughput": bitcoin_throughputs,
            "poem_latency": poem_latencies,
            "poem_optimal_g": poem_optimal_g,
            "poem_optimal_gamma": poem_optimal_gamma,
            "poem_throughput": poem_throughputs,
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
