from blocktree import Block, common_prefix_k
from numpy.random import binomial

class BackboneSimulation:
  def __init__(self, f, L, n, q):
    self.f = f
    self.L = L
    self.n = n
    self.q = q
    self.t = 0
    # self.kappa = 256

    # f ~= (n - t) * q * p
    # f = 1 - (1 - p)**(q * (n - t)) # Union bound: q * (n - t) * p
    # solving for p:
    self.p = 1 - (1 - self.f)**(1 / (self.q * (self.n - self.t)))
    # p = T / 2**kappa
    # solving for T:
    # self.T = self.p * 2**self.kappa

    self.genesis = Block()
    self.tips = [self.genesis]

  def simulate_execution(self):
    safe_common_prefix = 0

    for r in range(self.L):
      successes = self.simulate_round(r)

      # balancing attack
      new_tips = []
      for tip in self.tips:
        for i in range(successes // len(self.tips)):
          block = Block()
          tip.add_child(block)
          new_tips.append(block)
      for i in range(successes % len(self.tips)):
        block = Block()
        self.tips[i].add_child(block)
        new_tips.append(block)
      if new_tips:
        self.tips = new_tips

      # TODO: Use heavy–light decomposition to optimize this
      safe_common_prefix = max(safe_common_prefix, common_prefix_k(self.tips))

    return {
      'tips': self.tips,
      'common_prefix': safe_common_prefix,
    }

  def simulate_round(self, r):
    return binomial(self.n - self.t, 1 - (1 - self.p)**self.q)

kappa = 256
hash_rate = 1
q = 1
f = 1/2 # probability of successful round
L = 1000000
n = 1000

simulation = BackboneSimulation(f, L, n, q)
execution = simulation.simulate_execution()
tips = execution['tips']
common_prefix = execution['common_prefix']

print('Number of tips:', len(tips))
print('Number of leaves:', len(list(simulation.genesis.leaves())))
print('Height of the first chain:', tips[0].height)
print('Minimum safe common prefix:', common_prefix)
round_duration = 12 # seconds
print('Confirmation time:', common_prefix * round_duration / f, 'seconds')