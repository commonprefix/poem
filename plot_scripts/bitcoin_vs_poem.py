import argparse
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

plt.rcParams['text.usetex'] = True

def bitcoin_vs_poem(file_path):
    import json

    with open(file_path, 'r') as file:
        poem_data = json.load(file)


    monte_carlo = poem_data['monte_carlo']
    error = poem_data['error']
    beta_range = poem_data['beta']
    bitcoin_latency = poem_data['bitcoin_latency']
    poem_latency = poem_data['poem_latency']

    fig, ax1 = plt.subplots()
    ax1.semilogy(beta_range, bitcoin_latency, marker='.', linestyle='-', color='blue', label='Bitcoin')
    ax1.semilogy(beta_range, poem_latency, marker='.', linestyle='-', color='red', label='PoEM')
    ax1.set_xlabel(r'$\beta$')
    ax1.set_ylabel(r'Latency (in $\Delta$s)')
    ax1.tick_params(axis='y', labelcolor='black')
    ax1.legend()

    ax2 = ax1.twinx()
    ax2.set_ylabel(r'Speed improvement', color='green')
    ax2.plot(beta_range, [(bitcoin_latency[i]/poem_latency[i] - 1) * 100 for i in range(len(beta_range))],
             marker='.', linestyle='-', color='green', label='Speed Improvement')
    ax2.tick_params(axis='y', labelcolor='green')
    ax2.yaxis.set_major_formatter(FuncFormatter(lambda y, _: '{:.0f}\%'.format(y)))

    bitcoin_optimal_g = poem_data['bitcoin_optimal_g']
    poem_optimal_g = poem_data['poem_optimal_g']
    poem_optimal_gamma = poem_data['poem_optimal_gamma']

    ax3 = ax1.twinx()
    ax3.plot(beta_range, bitcoin_optimal_g, marker='.', linestyle='--', color='blue', label=rf'Bitcoin optimal $g$')
    ax3.plot(beta_range, poem_optimal_g, marker='.', linestyle='--', color='red', label=rf'PoEM optimal $g$')
    ax3.tick_params(axis='y')
    ax3.set_ylabel(r'$g$', color='black')
    ax3.legend()

    ax4 = ax1.twinx()
    ax4.plot(beta_range, poem_optimal_gamma, marker='.', linestyle='dashed', color='purple', label='PoEM optimal $\gamma$')
    ax4.tick_params(axis='y', labelcolor='purple')
    ax4.set_ylabel(r'$\gamma$', color='black')
    ax4.legend()


    plt.title(rf'PoEM vs Bitcoin latency based on adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.show()

def main():
    # Create the argument parser
    parser = argparse.ArgumentParser(description="Plots Bitcoin vs PoEM")

    # Add an argument for the string input
    parser.add_argument('--file-name', type=str, required=True, help="Simulation data file name.")

    # Parse the command-line arguments
    args = parser.parse_args()
    file_path = rf'../simulation/simulation_data/{args.file_name}'
    bitcoin_vs_poem(file_path)

if __name__ == "__main__":
    main()