#!/bin/bash

python3 bitcoin_vs_poem.py --g --file-name bitcoin_vs_poem_beta_0.01_0.45_33_g_0.05_85.0_50_gamma_0_20.0_40_monte_carlo_100000_error_0.1.json

python3 bitcoin_vs_poem.py --bitcoin-vs-poem --file-name bitcoin_vs_poem_beta_0.01_0.45_33_g_0.05_85.0_50_gamma_0_20.0_40_monte_carlo_100000_error_0.1.json

python3 g_latency.py --file-name poem_g_latency_beta_0.2_g_0.1_6.3_50_gamma_0_monte_carlo_100000_error_0.1.json