# This simulation is in continuous time
# and samples a Poisson stochastic process for
# both Bitcoin and PoEM and simulates the
# Everything is a Race private miner,
# calculating the honest subtree height VS
# the adversarial private chain height.
# In the end, we plot the delay of both systems
# vs the adversarial resilience.

from pprint import pprint
from collections import defaultdict, Counter
import numpy as np
import time

# from math import log, floor, e
# import graphviz

# dot = graphviz.Digraph(comment='The Honest Blocktree', engine='neato')
# dot.graph_attr['rankdir'] = 'RL'

inf = float('inf')

# f is the expected honest blocktree growth rate without the presence of an adversary
# g is Poisson lambda parameter; expected number of honest blocks produced per round
# g = 0.5
C = 1000 # desired honest block tree size in number of blocks, in expectation
# print('Chain size of', C)
# The adversarial chain is expected to grow at a rate of
# (t / (n - t)) * g = (1 - delta)g

# In the following simulation, we take n -> inf while keeping g constant
# This means that each block is produced by a different miner,
# and miners never get to extend their own blocks.
# We take the delay parameter to be Δ = 1, but we allow blocks to arrive
# at arbitrary continuous times (not just at integer multiples of Δ).

def simulate_honest(g, L, get_work=lambda: 1):
  # Simulate the production of the honest block tree without the presence of an adversary
  events = []
  block_time = 0
  block_index = 0
  while block_time < L:
    # Exponential distribution beta parameter is the expected
    # honest block interarrival rate 1/g
    # Sample the time at which the next block is produced by an honest party
    block_time += np.random.exponential(1/g)
    block_index += 1
    # Calculate the time at which this block will be received by all other honest parties
    block_arrival_time = block_time + 1 # Δ = 1
    events.append((block_time, 'mine', block_index))
    events.append((block_arrival_time, 'receive', block_index)) # the optimal adversary delays as much as allowed

  events.sort()
  # Invariant: heaviest_chain_weight is the weight of the heaviest chain as seen by
  # all honest parties who have not mined any blocks so far at the current time
  heaviest_chain_weight = 0
  heaviest_chain_tip = 0
  # dot.node('0', '0', pos=f'0,0!')
  weight_ending_at = {}
  weight_index = defaultdict(int)
  parent_of = {}
  time_of_weight = [0]

  # Go through all the block mining / reception events in chronological order
  for time, event_type, block_index in events:
    if time > L:
      break
    if event_type == 'mine':
      # print('mined block', block_index, 'at time', time)
      this_block_weight = get_work()
      # Extend the best known chain with this block
      new_chain_weight = heaviest_chain_weight + this_block_weight
      # The new chain weight is not known to the other parties yet,
      # but store it for a future update
      weight_index[new_chain_weight] += 1
      # f'{block_index} {time} {weight_index[new_chain_weight]}'
      # dot.node(str(block_index), str(block_index), pos=f'{time},{weight_index[new_chain_weight]}!')
      weight_ending_at[block_index] = new_chain_weight
      if new_chain_weight not in time_of_weight:
        time_of_weight.append(inf)
      time_of_weight[new_chain_weight] = min(time_of_weight[new_chain_weight], time)
      parent_of[block_index] = heaviest_chain_tip
      # dot.edge(str(block_index), str(heaviest_chain_tip))
    elif event_type == 'receive':
      # print('received block', block_index, 'at time', time)
      if heaviest_chain_weight < weight_ending_at[block_index]:
        heaviest_chain_weight = weight_ending_at[block_index]
        heaviest_chain_tip = block_index
      # heaviest_chain_weight = max(heaviest_chain_weight, weight_ending_at[block_index])
      # print('heaviest chain weight is now', heaviest_chain_weight)

  # print('heaviest chain weight is', heaviest_chain_weight)
  return time_of_weight

# MONTE_CARLO_ITERATIONS = 1000

def backbone_get_work():
  return 1

def simulate_adversary(g, L, get_work=lambda: 1):
  block_time = 0
  block_height = 0

  time_of_weight = [0]

  while block_time < L:
    block_time += np.random.exponential(1/g)
    block_height += get_work()
    time_of_weight.append(block_time)

  return time_of_weight

# def find_backbone_resilience(g, L):
#   for EiaR_beta in np.arange(0, 1, 0.01):
#     weights = 0
#     for _ in range(MONTE_CARLO_ITERATIONS):
#       weights += simulate(g, L, backbone_get_work)
#     # print('average backbone honest heaviest chain weight is:', weights / MONTE_CARLO_ITERATIONS)
#     average_honest_weight = weights / MONTE_CARLO_ITERATIONS
#     # delta = 1/2 # 30% adversary
#     average_adversarial_weight = L * g * EiaR_beta / (1 - EiaR_beta)
#     if average_adversarial_weight > average_honest_weight:
#       return EiaR_beta
#
# def poem_get_work():
#   return -log(np.random.uniform(0, 1), 2)
#
# def find_poem_resilience(g, L):
#   for EiaR_beta in np.arange(0, 1, 0.01):
#     weights = 0
#     for _ in range(MONTE_CARLO_ITERATIONS):
#       weights += simulate(g, L, poem_get_work)
#     # print('average poem honest heaviest chain weight is:', weights / MONTE_CARLO_ITERATIONS)
#     average_honest_weight = weights / MONTE_CARLO_ITERATIONS
#
#     adversarial_successes = L * g * EiaR_beta / (1 - EiaR_beta)
#     # weights = 0
#     # for _ in range(MONTE_CARLO_ITERATIONS):
#     #   adversarial_chain_work = 0
#     #   for adversarial_block in range(floor(adversarial_successes)):
#     #     adversarial_chain_work += poem_get_work()
#     #   weights += adversarial_chain_work
#     # # print('average poem adversarial chain weight is:', weights / MONTE_CARLO_ITERATIONS)
#     # average_adversarial_weight = weights / MONTE_CARLO_ITERATIONS
#     # print('PoEM avg adv weight approximation: ', average_adversarial_weight)
#
#     # But noting that:
#     # Σ_{μ=0}^{κ} μ * 2^{-μ - 1} ~=
#     # Σ_{μ=0}^{\inf} μ * 2^{-μ - 1} = 2, the above simplifies to
#     # average_adversarial_weight = 2 * floor(adversarial_successes)
#     # However, simulations show that large logs are exceedingly rare,
#     # and the average tends to be closer to 1.44... More investigation needed.
#     average_adversarial_weight = 1.44 * floor(adversarial_successes)
#     # print('PoEM adv weight direct expectation: ', average_adversarial_weight)
#
#     if average_adversarial_weight > average_honest_weight:
#       return EiaR_beta
#
# def monte_carlo(f, iterations):
#   weights = 0
#   for _ in range(iterations):
#     weights += f()
#   return weights / iterations
#
# for g in np.arange(0.01, 2, 0.01):
#   worst_case_f = g / (1 + g)
#   L = 2 * C / worst_case_f # simulation lifetime to meet this expectation
#   print('g =', g)
#   poem_chain_weight = monte_carlo(lambda: simulate(g, L, poem_get_work), MONTE_CARLO_ITERATIONS)
#   print('\tEmpirical PoEM honest chain growth rate: ', poem_chain_weight / L)
#

# start_time = time.time()

# for i in range(10000):
# pprint(simulate_honest(2, 100))

# c = Counter()
# for i in range(10000):
#   c[len(simulate_adversary(2, 100))] += 1
#
# # print('Elapsed time:', time.time() - start_time)
#
# import matplotlib.pyplot as plt
#
# plt.bar(c.keys(), c.values())
# plt.show()

# dot.render('doctest-output/honest-tree.gv', view=True)

H = np.array(simulate_honest(2, 100))
A = np.array(simulate_adversary(2, 100))

if H.size > A.size:
  A = np.pad(A, (0, H.size - A.size), 'constant', constant_values=(0, 0))
else:
  H = np.pad(H, (0, A.size - H.size), 'constant', constant_values=(0, 0))

print(A)
print(H)

p = A < H

print(p)