
def plot_gamma_latency(data, filename):
    import matplotlib.pyplot as plt
    plt.figure()
    plt.rcParams['text.usetex'] = True
    monte_carlo = data['monte_carlo']
    error = data['error']
    beta = data['beta']
    g = data['g']
    gamma_range = data['gamma']

    poem_latency = data['interpolated_poem_latency']
    bitcoin_latency = data['interpolated_bitcoin_latency']

    plt.axhline(y=bitcoin_latency, color='blue', linestyle='-', linewidth=0.8, label='Bitcoin')
    plt.plot(gamma_range, poem_latency, marker='.', linestyle='-', color='red', label='PoEM')

    plt.xlabel(r'$\gamma$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.legend()
    plt.savefig(filename, bbox_inches = "tight")
    plt.title(rf'PoEM latency per $\gamma$ compared to Bitcoin latency with $\beta = {beta}$ and $g = {g}$(MONTE_CARLO = {monte_carlo}, error = {error})', loc='center')
    # plt.show()
    plt.close()

import json
with open('../rust-simulation/simulation_data/gamma_latency_β_0.1_g_0.7_gamma_0..7.9:0.5+8..70:2_monte_carlo_100000.json', 'r') as file:
    data01 = json.load(file)

with open('../rust-simulation/simulation_data/gamma_latency_β_0.3_g_0.7_gamma_0..70:2_monte_carlo_100000.json', 'r') as file:
    data03 = json.load(file)

plot01 = plot_gamma_latency(data01, "gamma_latency_01.pdf")
plot03 = plot_gamma_latency(data03, "gamma_latency_03.pdf")
