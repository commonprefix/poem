use clap::{ArgGroup, Parser};
use rust_simulation::abstract_simulations::{compare_bitcoin_and_poem_one_sample, one_sample_bitcoin_performance};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("commands")
        .args(&["one_sample_bitcoin_performance", "compare_bitcoin_and_poem_one_sample"])
        .required(true)
        .multiple(false)
))]
struct Args {
    #[arg(long)]
    one_sample_bitcoin_performance: bool,
    #[arg(long)]
    compare_bitcoin_and_poem_one_sample: bool,
    #[arg(long)]
    beta: Option<f64>,
    #[arg(short)]
    g: Option<f64>,
    #[arg(long)]
    gamma: Option<f64>,
}

fn main() {
   let start = std::time::Instant::now();
    let args = Args::parse();

    if args.one_sample_bitcoin_performance {
        one_sample_bitcoin_performance(args.g.unwrap(), args.beta.unwrap());
    }

    if args.compare_bitcoin_and_poem_one_sample {
        compare_bitcoin_and_poem_one_sample(args.g.unwrap(), args.beta.unwrap(), args.gamma.unwrap());
    }

    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}
