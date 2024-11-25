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

def plot_g_latency(file_path):
    import json

    # Open and read the JSON file
    with open(file_path, 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta = data['beta'][0]
    g_range = data['g']
    gamma = data['gamma'][0]
    latency = data['latency']

    plt.plot(g_range, latency, marker='.', linestyle='-', color='red', label='PoEM')

    # plt.title(rf'PoEM latency per $g$ with $\beta = {beta}$ and $\gamma = {gamma}$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.xlabel(r'Block Production Rate $g$')
    plt.ylabel(r'Latency (in $\Delta$s)')

    # plt.legend()
    plt.savefig("g_latency.pdf", bbox_inches = "tight")
    # plt.show()

def main():
    # Create the argument parser
    parser = argparse.ArgumentParser(description="Plots PoEM g-latency")

    # Add an argument for the string input
    parser.add_argument('--file-name', type=str, required=True, help="Simulation data file name.")

    # Parse the command-line arguments
    args = parser.parse_args()
    file_path = rf'../simulation/simulation_data/{args.file_name}'

    plot_g_latency(file_path)

if __name__ == "__main__":
    main()