import numpy as np
from collections import deque
import heapq
import math


inf = float('inf')

entropy_λ = 1 / math.log(2, math.e)

def get_entropy():
  return np.random.exponential(entropy_λ)

def sample_honest(g, max_weight, get_work=lambda: 1):
  weight_improvements = [(0,0)] # (arrival_time, weight)
  block_time = 0
  heaviest_chain_weight = 0
  receive_events = deque()
  while weight_improvements[-1][1] < max_weight:
    block_time += np.random.exponential(1/g)

    # Before processing the newly mined block first process all received blocks before it
    while len(receive_events) > 0:
      arrival_time, weight = receive_events[0]
      if arrival_time > block_time:
        break
      heaviest_chain_weight = max(heaviest_chain_weight, weight)
      receive_events.popleft()

    block_arrival_time = block_time + 1 # Δ = 1
    this_block_weight = get_work()
    new_chain_weight = heaviest_chain_weight + this_block_weight
    receive_events.append((block_arrival_time, new_chain_weight)) # the optimal adversary delays as much as allowed

    if weight_improvements[-1][1] < new_chain_weight:
      weight_improvements.append((block_time, new_chain_weight))
  return weight_improvements

def sample_adversary(g, max_weight, get_work=lambda: 1):
  block_time = 0
  block_weight = 0

  weight_improvements = [(0, 0)] # (arrival_time, weight)

  while weight_improvements[-1][1] < max_weight:
    block_time += np.random.exponential(1/g)
    block_weight += get_work()
    weight_improvements.append((block_time, block_weight))

  return weight_improvements

def sample_all(g_range, max_weight, MONTE_CARLO = 1000, get_work=lambda: 1):
  adversary_samples = []
  for _ in range(MONTE_CARLO):
    adversary_samples.append(np.array(sample_adversary(1, max_weight, get_work)))

  g_to_honest_samples = {}

  for g in g_range:
    g_to_honest_samples[g] = []
    print("sampling for g", g)
    for _ in range(MONTE_CARLO):
      g_to_honest_samples[g].append(np.array(sample_honest(g, max_weight, get_work)))
  return g_to_honest_samples, adversary_samples

def plot_latency_k(g_list, latency_list, k_list, title, x_label):
  import matplotlib.pyplot as plt
  import numpy as np
  plt.rcParams['text.usetex'] = True

  # Plotting
  fig, ax1 = plt.subplots()
  ax1.plot(g_list, latency_list, marker='o', linestyle='-', color='blue', label='Latency')
  ax1.set_xlabel(x_label)
  ax1.set_ylabel(r'Latency ($\frac{1}{f}$)', color='b')
  ax1.tick_params(axis='y', labelcolor='b')

  ax2 = ax1.twinx()
  ax2.plot(g_list, k_list, marker='o', linestyle='-', label=r'$k$', color='red')
  ax2.set_ylabel(r'$k$', color='r')
  ax2.tick_params(axis='y', labelcolor='red')

  # plt.figure(figsize=(8, 5))
  plt.title(title)
  # plt.plot(g_list, latency_list, marker='o', linestyle='-', color='red')
  # plt.xlabel(r'$g$')
  # plt.ylabel(r'Latency ($\frac{1}{f}$)')
  # plt.grid(True)
  fig.tight_layout()
  plt.show()

def get_optimal_latency_and_k_for_beta(beta, g_to_honest_samples, adversary_samples, epsilon = 0.1):
  g_list = []
  latency_list = []
  k_list = []

  for g in list(g_to_honest_samples.keys()):
    print("working for g", g)
    k, latency = get_latency_and_k_for_g_and_beta(g, beta, g_to_honest_samples[g], adversary_samples, epsilon)

    if k == inf:
      break

    g_list.append(g)
    latency_list.append(latency)
    k_list.append(k)
  return g_list, latency_list, k_list

def get_k(honest_sample, adversary_sample):
  combined_samples = [] # (is_honest, weight)
  j = 0

  for time, weight in honest_sample:
    while j < len(adversary_sample) and adversary_sample[j][0] < time:
      combined_samples.append((False, adversary_sample[j][1]))
      j += 1
    combined_samples.append((True, weight))

  max_weight = 0
  max_weight_honest = False
  k = 0
  for is_honest, weight in combined_samples:
    if weight > max_weight:
      max_weight = weight
      if is_honest:
        if not max_weight_honest:
          k = weight
        max_weight_honest = True
      else:
        max_weight_honest = False

  if not max_weight_honest:
    return inf
  return k

# h = sample_honest(2, 400)
# a = sample_adversary(0.5, 400)
# print(h[:30])
# print(a[:30])
# print(get_k(h, a))

  # bitmask = honest_sample < adversary_sample
  # for height, honest_winning in reversed(list(enumerate(bitmask))):
  #   if not honest_winning:
  #     if height + 1 == len(honest_sample):
  #       return inf
  #     return height + 1

def get_latency_and_k_for_g_and_beta(g, beta, honest_samples, adversary_samples, epsilon = 0.1):
    print("Getting latency and k for g = ", g)
    potential_k = []
    L = 0

    monte_carlo = len(honest_samples)
    max_weight = len(honest_samples[0])
    for i in range(monte_carlo):
      h = honest_samples[i]
      a = list(map(lambda x: (x[0] * ((1 - beta) / beta) / g, x[1]), adversary_samples[i]))
      potential_k.append(get_k(h, a))
      L += h[-1][0]
    k = heapq.nlargest(int(monte_carlo * epsilon), potential_k)[-1]
    print(k)
    L = L / monte_carlo
    latency = k * (L / max_weight)
    print(latency)

    return k, latency


def plot_g_bitcoin(MONTE_CARLO, beta):
  g_to_honest_samples, adversary_samples = sample_all(np.arange(0.05, 4, 0.1), 400, MONTE_CARLO)
  g_list, latency_list, k_list = get_optimal_latency_and_k_for_beta(beta, g_to_honest_samples, adversary_samples)
  plot_title = rf'Bitcoin: Latency and k with adversarial resilience $\beta = {beta}$ based on $g$'
  x_label = r'$g$'
  plot_latency_k(g_list, latency_list, k_list, plot_title, x_label)

def plot_g_poem(MONTE_CARLO, beta):
  g_to_honest_samples, adversary_samples = sample_all(np.arange(0.05, 4, 0.1), 400, MONTE_CARLO, get_entropy)
  g_list, latency_list, k_list = get_optimal_latency_and_k_for_beta(beta, g_to_honest_samples, adversary_samples)
  plot_title = rf'PoEM: Latency and k with adversarial resilience $\beta = {beta}$ based on $g$'
  x_label = r'$g$'
  plot_latency_k(g_list, latency_list, k_list, plot_title, x_label)

def plot_beta_bitcoin(MONTE_CARLO):
  g_to_honest_samples, adversary_samples = sample_all(np.arange(0.05, 4, 0.1), 400, MONTE_CARLO)

  beta_list = np.arange(0.1, 0.4, 0.02)
  optimal_latency_list = []
  optimal_k_list = []

  for beta in beta_list:
    _, latency_list, k_list = get_optimal_latency_and_k_for_beta(beta, g_to_honest_samples, adversary_samples)
    optimal_latency = min(latency_list)
    optimal_k = k_list[latency_list.index(optimal_latency)]

    optimal_latency_list.append(optimal_latency)
    optimal_k_list.append(optimal_k)

  plot_title = rf'Bitcoin: Latency and k for various $\beta$.'
  x_label = r'$\beta$'
  # plot_latency_k(list(beta_list), optimal_latency_list, optimal_k_list, plot_title, x_label)

def plot_beta_poem(MONTE_CARLO):
  g_to_honest_samples, adversary_samples = sample_all(np.arange(0.05, 4, 0.1), 400, MONTE_CARLO, get_entropy)

  beta_list = np.arange(0.1, 0.4, 0.02)
  optimal_latency_list = []
  optimal_k_list = []

  for beta in beta_list:
    _, latency_list, k_list = get_optimal_latency_and_k_for_beta(beta, g_to_honest_samples, adversary_samples)
    optimal_latency = min(latency_list)
    optimal_k = k_list[latency_list.index(optimal_latency)]

    optimal_latency_list.append(optimal_latency)
    optimal_k_list.append(optimal_k)

  plot_title = rf'PoEM: Latency and k for various $\beta$.'
  x_label = r'$\beta$'
  plot_latency_k(list(beta_list), optimal_latency_list, optimal_k_list, plot_title, x_label)

plot_beta_bitcoin(10)
# plot_beta_poem(1000)
# plot_g_poem(1000, 0.2)
# plot_g_bitcoin(100, 0.2)
# simulate_and_plot_latency_k_g(0.3, 1000)