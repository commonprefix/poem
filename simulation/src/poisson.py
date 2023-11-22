import numpy as np
from pprint import pprint
from math import log, ceil, floor

# In Bitcoin Backbone notation, where:
#   p is the probability of a query being successful
#   n is the total number of parties
#   t is the number of adversarial parties
#   q is the number of queries per party per round
#   f is the probability of a round being successful
# we have:
# f = 1 - (1 - p)^{(n - t)q}
# f is the expected honest blocktree growth rate without the presence of an adversary
# g = (n - t)qp # expected number of honest blocks
# g is Poisson lambda parameter; expected number of honest blocks produced per round
# g = 0.5
C = 100 # desired honest block tree size in number of blocks, in expectation
print('Chain size of', C)
# The honest majority assumption states: t = (1 - delta)(n - t)
# The adversarial chain is expected to grow at a rate of
# (t / (n - t)) * g = (1 - delta)g

# In the following simulation, we take n -> inf while keeping g constant
# This means that each block is produced by a different miner,
# and miners never get to extend their own blocks.
# We take the delay parameter to be Δ = 1, but we allow blocks to arrive
# at arbitrary continuous times (not just at integer multiples of Δ).
# This is akin to taking q = 1, letting Δ be large, and counting
# the number of blocks within a Δ interval, then normalizing by Δ.

def simulate(g, L, get_work=lambda: 1):
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
  weight_ending_at = {}
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
      weight_ending_at[block_index] = new_chain_weight
    elif event_type == 'receive':
      # print('received block', block_index, 'at time', time)
      heaviest_chain_weight = max(heaviest_chain_weight, weight_ending_at[block_index])
      # print('heaviest chain weight is now', heaviest_chain_weight)

  # print('heaviest chain weight is', heaviest_chain_weight)
  return heaviest_chain_weight

MONTE_CARLO_ITERATIONS = 1000

def backbone_get_work():
  return 1

def find_backbone_resilience(g, L):
  for EiaR_beta in np.arange(0, 1, 0.01):
    weights = 0
    for _ in range(MONTE_CARLO_ITERATIONS):
      weights += simulate(g, L, backbone_get_work)
    # print('average backbone honest heaviest chain weight is:', weights / MONTE_CARLO_ITERATIONS)
    average_honest_weight = weights / MONTE_CARLO_ITERATIONS
    # delta = 1/2 # 30% adversary
    average_adversarial_weight = L * g * EiaR_beta / (1 - EiaR_beta)
    if average_adversarial_weight > average_honest_weight:
      return EiaR_beta

def poem_get_work():
  return -log(np.random.uniform(0, 1), 2)

def find_poem_resilience(g, L):
  for EiaR_beta in np.arange(0, 1, 0.01):
    weights = 0
    for _ in range(MONTE_CARLO_ITERATIONS):
      weights += simulate(g, L, poem_get_work)
    # print('average poem honest heaviest chain weight is:', weights / MONTE_CARLO_ITERATIONS)
    average_honest_weight = weights / MONTE_CARLO_ITERATIONS

    adversarial_successes = L * g * EiaR_beta / (1 - EiaR_beta)
    # weights = 0
    # for _ in range(MONTE_CARLO_ITERATIONS):
    #   adversarial_chain_work = 0
    #   for adversarial_block in range(floor(adversarial_successes)):
    #     adversarial_chain_work += poem_get_work()
    #   weights += adversarial_chain_work
    # # print('average poem adversarial chain weight is:', weights / MONTE_CARLO_ITERATIONS)
    # average_adversarial_weight = weights / MONTE_CARLO_ITERATIONS
    # print('PoEM avg adv weight approximation: ', average_adversarial_weight)

    # But noting that:
    # Σ_{μ=0}^{κ} μ * 2^{-μ - 1} ~=
    # Σ_{μ=0}^{\inf} μ * 2^{-μ - 1} = 2, the above simplifies to
    # average_adversarial_weight = 2 * floor(adversarial_successes)
    # However, simulations show that large logs are exceedingly rare,
    # and the average tends to be closer to 1.44... More investigation needed.
    average_adversarial_weight = 1.44 * floor(adversarial_successes)
    # print('PoEM adv weight direct expectation: ', average_adversarial_weight)

    if average_adversarial_weight > average_honest_weight:
      return EiaR_beta

# backbone_resiliences = []
# poem_resiliences = []
# 
def monte_carlo(f, iterations):
  weights = 0
  for _ in range(iterations):
    weights += f()
  return weights / iterations

for g in np.arange(0.01, 2, 0.01):
  L = C / g # simulation lifetime to meet this expectation
  print('g =', g)
  poem_chain_weight = monte_carlo(lambda: simulate(g, L, poem_get_work), MONTE_CARLO_ITERATIONS)
  print('\tPoEM honest chain growth rate: ', poem_chain_weight / L)

#   backbone_resilience = find_backbone_resilience(g, L)
#   print('\tBackbone resilience: ', backbone_resilience)
#   poem_resilience = find_poem_resilience(g, L)
#   print('\tPoEM resilience: ', poem_resilience)
#   backbone_resiliences.append(backbone_resilience)
#   poem_resiliences.append(poem_resilience)
#
# print('Backbone resiliences:')
# print(backbone_resiliences)
#
# print('PoEM resiliences:')
# print(poem_resiliences)
