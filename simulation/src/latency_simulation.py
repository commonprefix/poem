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
      time_of_weight.append(block_time)
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
  A = np.array(simulate_adversary(g * beta / (1 - beta), max_weight))
  # print(H)
  # print(A)

  L = H[max_weight - 1]
  return H < A, L

def get_latency_and_k(g, beta, max_weight = 50, MONTE_CARLO = 10000, error = 0.1):
  L = 0
  sum = [0] * max_weight
  for _ in range(MONTE_CARLO):
    s, l = simulate(g, beta, max_weight)
    # if not s[-1]:
    #   return inf, inf
    sum += s.astype(int)
    L += l

  L = L / MONTE_CARLO
  for height, amount in reversed(list(enumerate(sum))):
    # print("k= ", height + 1, amount)
    if amount < MONTE_CARLO * (1 - error):
      if height + 1 == max_weight:
        return inf, inf
      return (height + 1) * (L / max_weight), height + 1


#print(simulate(1, 0.4, 1000))
#print(get_latency(0.33, 0.4))


def plot_latency_resilience():
  import matplotlib.pyplot as plt
  import numpy as np
  plt.rcParams['text.usetex'] = True

  beta_list = []
  minimum_latency_list = []

  for b in np.arange(0.1, 0.4, 0.02):
    print("working for beta", b)
    g_list = np.arange(0.05, 5, 0.1)
    latency = []
    for g in g_list:
      l, k = get_latency_and_k(g, b, 400, 10)
      if l == inf:
        break
      latency.append(l)
    beta_list.append(b)
    minimum_latency_list.append(min(latency))


  # Plotting
  plt.figure(figsize=(8, 5))
  plt.plot(beta_list, minimum_latency_list, marker='o', linestyle='-', color='blue')
  plt.title(r'Latency based on adversarial resilience')
  plt.xlabel(r'Adversarial resilience ($\beta$)')
  plt.ylabel(r'Latency (tx/s)')
  plt.grid(True)
  plt.show()

def plot_latency_g(beta):
  import matplotlib.pyplot as plt
  import numpy as np
  plt.rcParams['text.usetex'] = True

  g_list = []
  latency_list = []
  k_list = []

  for g in np.arange(0.05, 6, 0.2):
    print("working for g", g)
    l, k = get_latency_and_k(g, beta, 400, 1000)
    if l == inf:
      break
    latency_list.append(l)
    k_list.append(k)
    g_list.append(g)

  # Plotting
  fig, ax1 = plt.subplots()
  ax1.plot(g_list, latency_list, marker='o', linestyle='-', color='blue', label='Latency')
  ax1.set_xlabel(r'$g$')
  ax1.set_ylabel(r'Latency ($\frac{1}{f}$)', color='b')
  ax1.tick_params(axis='y', labelcolor='b')

  ax2 = ax1.twinx()

  ax2.plot(g_list, k_list, marker='o', linestyle='-', label=r'$k$', color='red')
  ax2.set_ylabel(r'$k$', color='r')
  ax2.tick_params(axis='y', labelcolor='red')


  # plt.figure(figsize=(8, 5))
  plt.title(rf'Bitcoin: Latency and k with adversarial resilience $\beta = {beta}$ based on $g$')
  # plt.plot(g_list, latency_list, marker='o', linestyle='-', color='red')
  # plt.xlabel(r'$g$')
  # plt.ylabel(r'Latency ($\frac{1}{f}$)')
  # plt.grid(True)
  fig.tight_layout()
  plt.show()

for i in range(1, 10000):
  simulate_honest(3, 400)
# plot_latency_g(0.1)
# plot_latency_resilience()
# print(simulate(0.2, 0.4, 400))
# print(get_latency_and_k(0.4, 0.1, 400))