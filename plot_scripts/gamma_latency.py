import argparse
import matplotlib.pyplot as plt
from matplotlib import rc
from matplotlib.ticker import FuncFormatter

rc('text', usetex=True)
rc(
    'font',
    family='serif',
    serif=['Computer Modern Roman'],
    monospace=['Computer Modern Typewriter'],
    size=12
)

def plot_gamma_latency(file_path):
    import json

    # Open and read the JSON file
    with open(file_path, 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta = data['beta'][0]
    g = data['g'][0]
    gamma_range = data['gamma']
    poem_latency = data['poem_latency']
    bitcoin_latency = data['bitcoin_latency']
    print(bitcoin_latency)

    plt.figure()

    plt.axhline(y=bitcoin_latency, color='blue', linestyle='-', linewidth=0.9, label='Bitcoin')
    plt.plot(gamma_range, poem_latency, marker='.', linestyle='-', color='red', label='PoEM')

    plt.xlabel(r'Bias parameter $\gamma$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    plt.legend()
    plt.savefig(f"gamma_latency_beta_{beta}_g_{g}.pdf", bbox_inches = "tight")
    # plt.title(rf'PoEM latency per $\gamma$ compared to Bitcoin latency with $\beta = {beta}$ and $g = {g}$(MONTE_CARLO = {monte_carlo}, error = {error})', loc='center')
    # plt.show()
    # plt.close()

def main():
    # Create the argument parser
    parser = argparse.ArgumentParser(description="Plots PoEM gamma-latency")

    # Add an argument for the string input
    parser.add_argument('--file-name', type=str, required=True, help="Simulation data file name.")

    # Parse the command-line arguments
    args = parser.parse_args()
    file_path = rf'../simulation/simulation_data/{args.file_name}'

    plot_gamma_latency(file_path)

if __name__ == "__main__":
    main()