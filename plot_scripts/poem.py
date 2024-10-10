import argparse
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

plt.rcParams['text.usetex'] = True

def poem(file_path):
    import json

    with open(file_path, 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta_range = data['beta']
    latency = data['latency']
    optimal_k = data['optimal_k']
    optimal_g = data['optimal_g']
    optimal_gamma = data['optimal_gamma']
    throughput = data['throughput']
    max_work = data['max_work']
    max_height = data['max_height']
    adversary_max_work = data['adversary_max_work']
    adversary_max_height = data['adversary_max_height']

    print("k:", optimal_k)
    print("")
    print("g:", optimal_g)
    print("")
    print("max_height:", max_height)

    fig, ax1 = plt.subplots()
    ax1.semilogy(beta_range, latency, marker='.', linestyle='-', color='red', label='PoEM')
    ax1.set_xlabel(r'$\beta$')
    ax1.set_ylabel(r'Latency (in $\Delta$s)')
    ax1.tick_params(axis='y', labelcolor='black')
    ax1.legend()

    ax3 = ax1.twinx()
    ax3.semilogy(beta_range, optimal_g, marker='.', linestyle='--', color='blue', label=rf'PoEM optimal $g$')
    ax3.tick_params(axis='y')
    ax3.set_ylabel(r'$g$', color='black')
    ax3.legend()

    ax5 = ax1.twinx()
    ax5.plot(beta_range, optimal_k, marker='x', linestyle='dashed', color='orange', label=rf'PoEM optimal $k$')
    ax5.tick_params(axis='y')
    ax5.set_ylabel(r'$k$', color='black')
    ax5.legend()

    # ax6 = ax1.twinx()
    # ax6.plot(beta_range, max_work, marker='.', linestyle='dashed', color='blue', label=rf'honest max work')
    # ax6.plot(beta_range, adversary_max_work, marker='.', linestyle='dashed', color='red', label=rf'adversary max work')
    # ax6.plot(beta_range, optimal_k, marker='.', linestyle='-', color='orange', label=rf'k')
    # ax6.tick_params(axis='y')
    # ax6.set_ylabel(r'$work$', color='black')
    # ax6.legend()

    ax4 = ax1.twinx()
    ax4.plot(beta_range, optimal_gamma, marker='.', linestyle='dashed', color='purple', label='PoEM optimal $\gamma$')
    ax4.tick_params(axis='y', labelcolor='purple')
    ax4.set_ylabel(r'$\gamma$', color='black')
    ax4.legend()

    # ax5 = ax1.twinx()
    # ax5.plot(beta_range, bitcoin_throughput, marker='x', linestyle='--', color='blue', label=rf'Bitcoin throughput')
    # ax5.plot(beta_range, poem_throughput, marker='x', linestyle='--', color='red', label=rf'PoEM throughput')
    # ax5.tick_params(axis='y')
    # ax5.set_ylabel(rf'Throughput (in blocks/$\Delta$)', color='black')
    # ax5.legend()


    plt.title(rf'PoEM based on adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    plt.show()

def main():
    # Create the argument parser
    parser = argparse.ArgumentParser(description="Plots PoEM")

    # Add an argument for the string input
    parser.add_argument('--file-name', type=str, required=True, help="Simulation data file name.")

    # Parse the command-line arguments
    args = parser.parse_args()
    file_path = rf'../simulation/simulation_data/{args.file_name}'
    poem(file_path)

if __name__ == "__main__":
    main()