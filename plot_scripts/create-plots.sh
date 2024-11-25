#!/bin/bash

# Plot PoEM vs Bitcoin latency
python3 bitcoin_vs_poem.py --bitcoin-vs-poem --file-name bitcoin_vs_poem_beta_0.01_0.45_33_g_0.05_85.0_50_gamma_0_20.0_40_monte_carlo_100000_error_0.1.json

# Plot PoEM vs Bitcoin g
python3 bitcoin_vs_poem.py --g --file-name bitcoin_vs_poem_beta_0.01_0.45_33_g_0.05_85.0_50_gamma_0_20.0_40_monte_carlo_100000_error_0.1.json


# Plot latency vs g
python3 g_latency.py --file-name poem_g_latency_beta_0.2_g_0.1_6.3_50_gamma_0_monte_carlo_100000_error_0.1.json

# Plot latency vs gamma
python3 gamma_latency.py --file-name poem_gamma_latency_beta_0.1_g_1.7_gamma_0:70:20_monte_carlo_100000_error_0.1.json
python3 gamma_latency.py --file-name poem_gamma_latency_beta_0.3_g_0.4_gamma_0:70:35_monte_carlo_100000_error_0.1.json