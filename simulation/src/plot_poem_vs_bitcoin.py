import matplotlib.pyplot as plt
import numpy as np
plt.rcParams['text.usetex'] = True

def plot_gamma_latency():
    import json

    # Open and read the JSON file
    with open('../../rust-simulation/simulation_data/gamma_latency_β_0.3_g_0.7_gamma_0..70:2_monte_carlo_10000.json', 'r') as file:
        poem_data = json.load(file)

    with open('../../rust-simulation/simulation_data/bitcoin_latency_beta_0.3..0.3:0.01_monte_carlo_1000000.json', 'r') as file:
        bitcoin_data = json.load(file)

    monte_carlo = poem_data['monte_carlo']
    error = poem_data['error']
    beta = poem_data['beta']
    g = poem_data['g']
    gamma_range = poem_data['gamma']
    latency = poem_data['latency']

    bitcoin_latency = bitcoin_data['latency'][0]

    plt.axhline(y=bitcoin_latency, color='blue', linestyle='--', linewidth=1, label='Bitcoin') # Plot Bitcoin latency
    plt.plot(gamma_range, latency, marker='o', linestyle='-', color='green', label='PoEM')

    plt.title(rf'PoEM latency per $\gamma$ compared to Bitcoin latency with $\beta = {beta}$ and $g = {g}$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.xlabel(r'$\gamma$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.legend()

    plt.show()


def plot_bitcoin_latency():
    import json

    # Open and read the JSON file
    with open('../../rust-simulation/simulation_data/bitcoin_latency_monte_carlo_100000.json', 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta_range = data['beta']
    latency = data['latency']

    plt.plot(beta_range, latency, marker='o', linestyle='-', color='blue')
    plt.title(rf'Bitcoin: Optimal latency per adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.xlabel(r'$\beta$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.show()

def plot_poem_latency():
    import json

    # Open and read the JSON file
    with open('../../rust-simulation/simulation_data/poem_latency_monte_carlo_100.json', 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta_range = data['beta']
    latency = data['latency']

    plt.plot(beta_range, latency, marker='o', linestyle='-', color='red')
    plt.title(rf'PoEM: Optimal latency per adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.xlabel(r'$\beta$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.show()

def plot_poem_vs_bitcoin():
    import json

    # Open and read the JSON file
    with open('../../rust-simulation/simulation_data/poem_latency_monte_carlo_1000.json', 'r') as file:
        poem_data = json.load(file)

    with open('../../rust-simulation/simulation_data/bitcoin_latency_monte_carlo_1000.json', 'r') as file:
        bitcoin_data = json.load(file)

    bitcoin_latency = bitcoin_data['latency']
    poem_latency = poem_data['latency']
    beta_range = bitcoin_data['beta']

    plt.plot(beta_range, poem_latency, marker='o', linestyle='-', color='red', label='PoEM')
    plt.plot(beta_range, bitcoin_latency, marker='x', linestyle='-', color='blue', label='Bitcoin')

    plt.title(r'PoEM vs Bitcoin latency based on adversarial resilience $\beta$')
    plt.xlabel(r'$\beta$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.legend()

    plt.show()

def plot_optimal_poem():
    import json

    with open('../../rust-simulation/simulation_data/poem_latency_monte_carlo_1000.json', 'r') as file:
        poem_data = json.load(file)

    print(poem_data)
    monte_carlo = poem_data['monte_carlo']
    error = poem_data['error']
    beta_range = poem_data['beta']
    latency = poem_data['latency']
    throughput = poem_data['throughput']
    g_range = poem_data['optimal_g']
    gamma_range = poem_data['optimal_gamma']


    # Plotting
    fig, ax1 = plt.subplots()
    ax1.plot(beta_range, latency, marker='o', linestyle='-', color='blue', label='Latency')
    ax1.set_xlabel(r'$\beta$')
    ax1.set_ylabel(r'Latency (in $\Delta$s)', color='b')
    ax1.tick_params(axis='y', labelcolor='b')

    ax2 = ax1.twinx()
    ax2.plot(beta_range, g_range, marker='x', linestyle='dashed', label=r'$g$', color='red')
    ax2.set_ylabel(r'$g$', color='r')
    ax2.tick_params(axis='y', labelcolor='red')

    ax2 = ax1.twinx()
    ax2.plot(beta_range, gamma_range, marker='1', linestyle='dotted', label=r'$\gamma$', color='green')
    ax2.set_ylabel(r'$\gamma$', color='g')
    ax2.tick_params(axis='y', labelcolor='green')

    plt.title(rf'PoEM: Optimal parametrization per $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    fig.tight_layout()
    plt.show()

plot_gamma_latency()
# plot_bitcoin_latency()
# plot_poem_latency()
# plot_poem_vs_bitcoin()
# plot_optimal_poem()
