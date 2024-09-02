import numpy as np

inf = float('inf')

def simulate_honest(g, max_weight, get_work=lambda: 1):
  time_of_weight = [0]
  block_time = 0
  heaviest_chain_weight = 0
  receive_events = []
  while len(time_of_weight) < max_weight:
    block_time += np.random.exponential(1/g)

    # Before processing the newly mined block first process all received blocks before it
    while len(receive_events) > 0:
      arrival_time, weight = receive_events[0]
      if arrival_time > block_time:
        break
      heaviest_chain_weight = max(heaviest_chain_weight, weight)
      receive_events.pop(0)

    block_arrival_time = block_time + 1 # Δ = 1
    this_block_weight = get_work()
    new_chain_weight = heaviest_chain_weight + this_block_weight
    receive_events.append((block_arrival_time, new_chain_weight)) # the optimal adversary delays as much as allowed

    if len(time_of_weight) <= new_chain_weight:
      time_of_weight.append(inf)
    # Extend the best known chain with this block
    time_of_weight[new_chain_weight] = min(time_of_weight[new_chain_weight], block_time)
  return time_of_weight

def simulate_adversary(g, max_weight, get_work=lambda: 1):
  block_time = 0
  block_weight = 0

  time_of_weight = [0]

  while len(time_of_weight) < max_weight:
    block_time += np.random.exponential(1/g)
    block_weight += get_work()
    time_of_weight.append(block_time)

  return time_of_weight

def simulate(g, beta, max_weight):
  H = np.array(simulate_honest(g, max_weight))
  A = np.array(simulate_adversary(g * beta, max_weight))
  # print(H)
  # print(A)

  L = H[max_weight - 1]
  return H < A, L

def get_latency(g, beta, max_weight = 50, MONTE_CARLO = 10000, error = 0.1):
  L = 0
  sum = [0] * max_weight
  for _ in range(MONTE_CARLO):
    s, l = simulate(g, beta, max_weight)
    sum += s.astype(int)
    L += l

  L = L / MONTE_CARLO
  for height, amount in reversed(list(enumerate(sum))):
    if amount < MONTE_CARLO * (1 - error):
      return (height + 1) * (L / max_weight)


#print(simulate(1, 0.4, 1000))
#print(get_latency(0.33, 0.4))


def plot_latency():
  import matplotlib.pyplot as plt
  import numpy as np
  plt.rcParams['text.usetex'] = True

  beta_list = np.arange(0.1, 0.4, 0.02)
  minimum_latency_list = []

  for beta in beta_list:
    print("working for beta", beta)
    g_list = np.arange(0.05, 5, 0.1)
    latency = []
    for g in g_list:
      latency.append(get_latency(g, beta, 400, 100))
    minimum_latency_list.append(min(latency))


  # Plotting
  plt.figure(figsize=(8, 5))
  plt.plot(beta_list, minimum_latency_list, marker='o', linestyle='-', color='blue')
  plt.title(r'Latency based on adversarial resilience')
  plt.xlabel(r'Adversarial resilience ($\beta$)')
  plt.ylabel(r'Latency (tx/s)')
  plt.grid(True)
  plt.show()

plot_latency()
# print(simulate(1.5, 0.4, 400))