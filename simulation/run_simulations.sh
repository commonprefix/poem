#! /bin/bash

# PoEM vs Bitcoin simulation
cargo run --bin simulation --release -- --bitcoin-vs-poem --g-range 0.05:85.0:50 --beta-range 0.01:0.45:33 --gamma-range 0:20.0:40 --monte-carlo 100000

# Latency vs g simulation
cargo run --bin simulation --release -- --bitcoin-vs-poem --g-range 0.05:85.0:50 --beta-range 0.01:0.45:33 --gamma-range 0:20.0:40 --monte-carlo 100000

# Latency vs gamma simulations
cargo run --bin simulation --release -- --gamma-latency --g 1.7 --beta 0.1 --gamma-range 0:70:20 --monte-carlo 100000
cargo run --bin simulation --release -- --gamma-latency --g 0.4 --beta 0.3 --gamma-range 0:70:35 --monte-carlo 100000