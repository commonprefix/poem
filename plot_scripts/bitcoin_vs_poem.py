import argparse
import matplotlib.pyplot as plt
from matplotlib import rc
from matplotlib.ticker import FuncFormatter

plt.rcParams['text.usetex'] = True

# plt.rcParams['text.latex.preamble'] = [r'\usepackage{lmodern}']

rc('text', usetex=True)
rc(
    'font',
    family='serif',
    serif=['Computer Modern Roman'],
    monospace=['Computer Modern Typewriter'],
    size=12
)

def g(file_path):
    import json

    with open(file_path, 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta_range = data['beta'][:-3   ]
    bitcoin_latency = data['bitcoin_latency'][:-3]
    bitcoin_optimal_k = data['bitcoin_optimal_k'][:-3]
    poem_latency = data['poem_latency'][:-3]
    poem_optimal_k = data['poem_optimal_k'][:-3]
    # bitcoin_throughput = data['bitcoin_throughput']
    # poem_throughput = data['poem_throughput']
    bitcoin_max_work = data['bitcoin_max_work']
    poem_max_work = data['poem_max_work']
    bitcoin_adversary_max_work = data['bitcoin_adversary_max_work']
    poem_adversary_max_work = data['poem_adversary_max_work']



    fig, ax1 = plt.subplots()
    # ax1.semilogy(beta_range, bitcoin_latency, marker='.', linestyle='-', color='blue', label='Bitcoin')
    # ax1.semilogy(beta_range, poem_latency, marker='.', linestyle='-', color='red', label='PoEM')
    ax1.set_xlabel(r'Adversarial ratio $\beta$')
    # ax1.set_ylabel(r'Latency (in $\Delta$s)')
    # ax1.tick_params(axis='y', labelcolor='black')
    # ax1.legend(loc='upper center')

    # ax2 = ax1.twinx()
    # ax2.set_ylabel(rf'Latency Improvement (\%)', color='green')
    # ax2.plot(beta_range, [(bitcoin_latency[i]/poem_latency[i] - 1) * 100 for i in range(len(beta_range))],
    #          marker='.', linestyle='-', color='green', label=rf'Speed-up (\%)')
    # ax2.tick_params(axis='y', labelcolor='green')
    # ax2.yaxis.set_major_formatter(FuncFormatter(lambda y, _: '{:.0f}\%'.format(y)))
    # ax2.legend(loc='center right')

    print((bitcoin_latency[-1]/poem_latency[-1] - 1) * 100)

    bitcoin_optimal_g = data['bitcoin_optimal_g'][:-3]
    poem_optimal_g = data['poem_optimal_g'][:-3]
    poem_optimal_gamma = data['poem_optimal_gamma'][:-3]

    # # ax7 = ax1.twinx()
    # ax6.plot(beta_range , poem_max_work, marker='.', linestyle='-', color='red', label=rf'poem honest max work')
    # ax6.plot(beta_range, poem_adversary_max_work, marker='x', linestyle='dashed', color='red', label=rf'poem adversary max work')
    # ax6.plot(beta_range, bitcoin_max_work, marker='.', linestyle='-', color='blue', label=rf'bitcoin honest max work')
    # ax6.plot(beta_range, bitcoin_adversary_max_work, marker='.', linestyle='dashed', color='blue', label=rf'bitcoin adversary max work')

    # ax6 = ax1.twinx()
    # ax6.plot(beta_range, bitcoin_optimal_k, marker='x', linestyle='-', color='orange', label=rf'Bitcoin optimal $k$')
    # ax6.plot(beta_range, poem_optimal_k, marker='x', linestyle='dashed', color='orange', label=rf'PoEM optimal $k$')
    # ax6.tick_params(axis='y')
    # ax6.set_ylabel(r'$work$', color='black')
    # ax6.legend()

    # ax1 = ax1.twinx()
    ax1.semilogy(beta_range, bitcoin_optimal_g, marker='.', linestyle='--', color='blue', label=rf'Bitcoin')
    ax1.semilogy(beta_range, poem_optimal_g, marker='.', linestyle='-', color='red', label=rf'PoEM')
    ax1.tick_params(axis='y')
    ax1.set_ylabel(r'Optimal Block Production Rate $g$', color='black')
    ax1.legend()

    # ax5 = ax1.twinx()
    # ax5.plot(beta_range, bitcoin_optimal_k, marker='x', linestyle='dashed', color='blue', label=rf'Bitcoin optimal $k$')
    # ax5.plot(beta_range, [x / 1.44 for x in poem_optimal_k], marker='x', linestyle='dashed', color='red', label=rf'PoEM optimal $k$')
    # ax5.tick_params(axis='y')
    # ax5.set_ylabel(r'$k$', color='black')
    # ax5.legend()

    # ax4 = ax1.twinx()
    # ax4.plot(beta_range, poem_optimal_gamma, marker='.', linestyle='dashed', color='purple', label='PoEM optimal $\gamma$')
    # ax4.tick_params(axis='y', labelcolor='purple')
    # ax4.set_ylabel(r'$\gamma$', color='black')
    # ax4.legend()

    # ax5 = ax1.twinx()
    # ax5.plot(beta_range, bitcoin_throughput, marker='x', linestyle='--', color='blue', label=rf'Bitcoin throughput')
    # ax5.plot(beta_range, poem_throughput, marker='x', linestyle='--', color='red', label=rf'PoEM throughput')
    # ax5.tick_params(axis='y')
    # ax5.set_ylabel(rf'Throughput (in blocks/$\Delta$)', color='black')
    # ax5.legend()


    # plt.title(rf'PoEM vs Bitcoin latency based on adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    # plt.show()
    plt.savefig("optimal_g.pdf", bbox_inches = "tight")

def bitcoin_vs_poem(file_path):
    import json

    with open(file_path, 'r') as file:
        data = json.load(file)

    monte_carlo = data['monte_carlo']
    error = data['error']
    beta_range = data['beta'][:-3   ]
    bitcoin_latency = data['bitcoin_latency'][:-3]
    bitcoin_optimal_k = data['bitcoin_optimal_k'][:-3]
    poem_latency = data['poem_latency'][:-3]
    poem_optimal_k = data['poem_optimal_k'][:-3]
    # bitcoin_throughput = data['bitcoin_throughput']
    # poem_throughput = data['poem_throughput']
    bitcoin_max_work = data['bitcoin_max_work']
    poem_max_work = data['poem_max_work']
    bitcoin_adversary_max_work = data['bitcoin_adversary_max_work']
    poem_adversary_max_work = data['poem_adversary_max_work']



    fig, ax1 = plt.subplots()
    ax1.semilogy(beta_range, bitcoin_latency, marker='.', linestyle='--', color='blue', label='Bitcoin')
    ax1.semilogy(beta_range, poem_latency, marker='.', linestyle='-', color='red', label='PoEM')
    ax1.set_xlabel(r'Adversarial ratio $\beta$')
    ax1.set_ylabel(r'Latency (in $\Delta$s)')
    ax1.tick_params(axis='y', labelcolor='black')
    ax1.legend(loc='upper center')

    ax2 = ax1.twinx()
    ax2.set_ylabel(rf'Latency Improvement (\%)', color='green')
    ax2.plot(beta_range, [(bitcoin_latency[i]/poem_latency[i] - 1) * 100 for i in range(len(beta_range))],
             marker='.', linestyle='dotted', color='green', label=rf'Speed-up (\%)')
    ax2.tick_params(axis='y', labelcolor='green')
    ax2.yaxis.set_major_formatter(FuncFormatter(lambda y, _: '{:.0f}\%'.format(y)))
    ax2.legend(loc='center right')

    print((bitcoin_latency[-1]/poem_latency[-1] - 1) * 100)

    bitcoin_optimal_g = data['bitcoin_optimal_g'][:-3]
    poem_optimal_g = data['poem_optimal_g'][:-3]
    poem_optimal_gamma = data['poem_optimal_gamma'][:-3]

    # plt.title(rf'PoEM vs Bitcoin latency based on adversarial resilience $\beta$ (MONTE_CARLO = {monte_carlo}, error = {error})')
    # plt.show()
    plt.savefig("bitcoin_vs_poem.pdf", bbox_inches = "tight")

def main():
    # Create the argument parser
    parser = argparse.ArgumentParser(description="Plots Bitcoin vs PoEM")

    # Add an argument for the string input
    parser.add_argument('--file-name', type=str, required=True, help="Simulation data file name.")
    parser.add_argument('--g', action='store_true', help="Plot g")
    parser.add_argument('--bitcoin-vs-poem', action='store_true', help="Plot bitcoin vs poem")

    # Parse the command-line arguments
    args = parser.parse_args()
    file_path = rf'../simulation/simulation_data/{args.file_name}'

    if args.bitcoin_vs_poem:
        bitcoin_vs_poem(file_path)
    if args.g:
        g(file_path)

if __name__ == "__main__":
    main()