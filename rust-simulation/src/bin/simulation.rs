use std::{fs::File, io::Write};

use clap::{ArgGroup, Parser};
use rust_simulation::simulations::{latency_per_gamma_fixed_g};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("commands")
        .args(&["gamma_latency"])
        .required(true)
        .multiple(false)
))]
struct Args {
    #[arg(long)]
    gamma_latency: bool,
    #[arg(long)]
    beta: Option<f64>,
    #[arg(short)]
    g: Option<f64>,
    #[arg(long, default_value = "1000")]
    monte_carlo: usize,
    #[arg(long, default_value = "0.1")]
    error: f64,
}

fn main() {
    let args = Args::parse();
    println!(
        "Simulating with MONTE_CARLO = {} and error = {}",
        args.monte_carlo, args.error
    );

    if args.gamma_latency {
        if args.beta.is_some() && args.g.is_some() {
            println!(
                "Finding latency for various γ (fixed β = {} and g = {})",
                args.beta.unwrap(),
                args.g.unwrap()
            );
            let mut file = File::create(format!("simulation_data/gamma_latency_β_{}_g_{}.json", args.beta.unwrap(), args.g.unwrap())).unwrap();

            let mut gamma_range: Vec<f64> = (1..=100).map(|x| x as f64 * 0.02).collect();
            // let mut gamma_range: Vec<f64> = (1..=49).map(|x| x as f64 * 0.02).collect();
            // gamma_range.extend((1..=3).map(|x| x as f64).collect::<Vec<f64>>());

            let latency_per_gamma = latency_per_gamma_fixed_g(
                args.monte_carlo as i32,
                args.error,
                args.g.unwrap(),
                vec![args.beta.unwrap()],
                gamma_range.clone(),
            );

            let formatted_latency_per_gamma: Vec<f64> = latency_per_gamma
                .iter()
                .map(|x| x[0])
                .collect();

            println!("γ: {:?}", gamma_range);
            println!("Latency: {:?}", formatted_latency_per_gamma);

            let data = json!({
                "monte_carlo": args.monte_carlo,
                "error": args.error,
                "beta": args.beta.unwrap(),
                "g": args.g.unwrap(),
                "gamma": gamma_range,
                "latency": formatted_latency_per_gamma
            });
            let json_string = serde_json::to_string_pretty(&data).unwrap();

            file.write_all(json_string.as_bytes()).unwrap();
        }
    }
}
