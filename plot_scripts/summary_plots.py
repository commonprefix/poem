import matplotlib.pyplot as plt
import numpy as np
plt.rcParams['text.usetex'] = True

def plot_gamma_latency():
    import json

    # Open and read the JSON file
    with open('../../rust-simulation/simulation_data/gamma_latency_β_0.1_g_0.7_gamma_0..7.9:0.5+8..70:2_monte_carlo_100000.json', 'r') as file:
        data = json.load(file)
    # with open('../../rust-simulation/simulation_data/gamma_latency_β_0.3_g_0.7_gamma_0..70:2_monte_carlo_100000.json', 'r') as file:
    #     data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta = data['beta']
    g = data['g']
    gamma_range = data['gamma']
    latency = data['poem_latency']
    interpolated_latency = data['interpolated_poem_latency']

    bitcoin_latency = data['bitcoin_latency']
    interpolated_bitcoin_latency = data['interpolated_bitcoin_latency']

    plt.axhline(y=interpolated_bitcoin_latency, color='blue', linestyle='-', linewidth=0.8, label='Bitcoin')
    plt.plot(gamma_range, interpolated_latency, marker='.', linestyle='-', color='red', label='PoEM')

    # plt.axhline(y=bitcoin_latency, color='blue', linestyle='--', linewidth=1, label='Bitcoin not interpolated')
    # plt.plot(gamma_range, latency, marker='o', linestyle='--', color='red', label='PoEM not interpolated')

    # plt.title(rf'PoEM latency per $\gamma$ compared to Bitcoin latency with $\beta = {beta}$ and $g = {g}$(MONTE_CARLO = {monte_carlo}, error = {error})', loc='center')
    # plt.title(rf'PoEM latency per $\gamma$ compared to Bitcoin latency with $\beta = {beta}$ and $g = {g}$', y=1.05)
    plt.xlabel(r'$\gamma$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.legend()

    # plt.show()
    plt.savefig("gamma_latency.pdf", bbox_inches = "tight")

def plot_g_latency():
    import json

    # Open and read the JSON file
    with open('../../rust-simulation/simulation_data/g_latency_β_0.2_g_0.1:6.3:0.1_gamma_0_monte_carlo_100000.json', 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta = data['beta']
    g_range = data['g']
    gamma = data['gamma']
    latency = data['poem_latency']

    plt.plot(g_range, latency, marker='.', linestyle='-', color='red', label='PoEM')

    # plt.title(rf'PoEM latency per $g$ with $\beta = {beta}$ and $\gamma = {gamma}$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.xlabel(r'$g$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    # plt.legend()
    plt.savefig("g_latency.pdf", bbox_inches = "tight")
    # plt.show()


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

    with open('../../rust-simulation/simulation_data/poem_performance_β_0.01..0.4:0.01_g_0.1..5:0.1_gamma_0.1..5:0.5_monte_carlo_1000.json', 'r') as file:
        poem_data = json.load(file)

    monte_carlo = poem_data['monte_carlo']
    error = poem_data['error']
    beta_range = poem_data['beta']
    latency = poem_data['poem_latency']
    optimal_gamma = poem_data['optimal_gamma']
    optimal_g = poem_data['optimal_g']


    # Plotting
    fig, ax1 = plt.subplots()
    ax1.plot(beta_range, latency, marker='o', linestyle='-', color='red', label='Latency')
    ax1.set_xlabel(r'$\beta$')
    ax1.set_ylabel(r'Latency (in $\Delta$s)', color='b')
    ax1.tick_params(axis='y', labelcolor='b')

    ax2 = ax1.twinx()
    ax2.plot(beta_range, optimal_g, marker='x', linestyle='dashed', label=r'$g$', color='red')
    ax2.set_ylabel(r'$g$', color='r')
    ax2.tick_params(axis='y', labelcolor='red')

    ax2 = ax1.twinx()
    ax2.plot(beta_range, optimal_gamma, marker='1', linestyle='dotted', label=r'$\gamma$', color='green')
    ax2.set_ylabel(r'$\gamma$', color='g')
    ax2.tick_params(axis='y', labelcolor='green')

    plt.title(rf'PoEM: Optimal parametrization per $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    fig.tight_layout()
    plt.show()

def compare_bitcoin_and_poem_one_sample():
    import json

    with open('../../rust-simulation/simulation_data/compare_bitcoin_and_poem_one_sample_g_0.7_beta_0.3_gamma_10000000.json', 'r') as file:
        poem_data = json.load(file)

    bitcoin_honest_time = poem_data['bitcoin_honest_time']
    bitcoin_honest_work = poem_data['bitcoin_honest_work']
    bitcoin_adversary_time = poem_data['bitcoin_adversary_time']
    bitcoin_adversary_work = poem_data['bitcoin_adversary_work']
    poem_honest_time = poem_data['poem_honest_time']
    poem_honest_work = poem_data['poem_honest_work']
    poem_adversary_time = poem_data['poem_adversary_time']
    poem_adversary_work = poem_data['poem_adversary_work']


    # Plotting
    fig, ax1 = plt.subplots()
    ax1.step(bitcoin_honest_time, bitcoin_honest_work, marker='o', linestyle='-', color='blue', label='Bitcoin honest', where='post')
    ax1.step(bitcoin_adversary_time, bitcoin_adversary_work, marker='o', linestyle='-', color='red', label='Bitcoin adversary', where='post')
    ax1.set_xlabel(r'time')
    ax1.set_ylabel(r'Bitcoin work', color='b')
    ax1.tick_params(axis='y', labelcolor='b')

    ax2 = ax1.twinx()
    ax2.step(poem_honest_time, poem_honest_work, marker='x', linestyle='dashed', color='blue', label='PoEM honest', where='post')
    ax2.step(poem_adversary_time, poem_adversary_work, marker='x', linestyle='dashed', color='red', label='PoEM adversary', where='post')
    ax1.set_ylabel(r'Poem work', color='b')
    ax1.tick_params(axis='y', labelcolor='b')

    plt.title(rf'Compare same execution Bitcoin and PoEM')
    fig.tight_layout()
    plt.show()

def bitcoin_vs_poem():
    import json

    with open('../../rust-simulation/simulation_data/bitcoin_vs_poem_β_0.01..0.4:0.01_g_0.1..5:0.1_gamma_0.1..8:0.3_monte_carlo_1000.json', 'r') as file:
        poem_data = json.load(file)


    monte_carlo = poem_data['monte_carlo']
    error = poem_data['error']
    beta_range = poem_data['beta']
    bitcoin_latency = poem_data['bitcoin_latency']
    poem_latency = poem_data['poem_latency']
    large_gamma_poem_latency = poem_data['large_gamma_poem_latency']
    zero_gamma_poem_latency = poem_data['zero_gamma_poem_latency']

    plt.plot(beta_range, poem_latency, marker='o', linestyle='-', color='red', label='PoEM')
    plt.plot(beta_range, bitcoin_latency, marker='x', linestyle='-', color='blue', label='Bitcoin')
    plt.plot(beta_range, large_gamma_poem_latency, marker='.', linestyle='-', color='green', label='PoEM large gamma')
    plt.plot(beta_range, zero_gamma_poem_latency, marker='.', linestyle='-', color='orange', label='PoEM zero gamma')

    plt.title(rf'PoEM vs Bitcoin latency based on adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.xlabel(r'$\beta$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.legend()
    plt.show()

# plot_gamma_latency()
# plot_g_latency()
# plot_bitcoin_latency()
# plot_poem_latency()
# plot_poem_vs_bitcoin()
# plot_optimal_poem()
# compare_bitcoin_and_poem_one_sample()
bitcoin_vs_poem()