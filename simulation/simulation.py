from blocktree import Block, common_prefix_k
from random import randint, random
from numpy.random import binomial

class Simulation:
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

    return self.tips

  def simulate_round(self, r):
    return binomial(self.n - self.t, 1 - (1 - self.p)**self.q)

kappa = 256
round_duration = 12 # seconds
tera = 10**12
hash_rate = 300 * tera # per second hash rate of one ASIC machine in SHA256
q = hash_rate * round_duration # number of queries per round
f = 1/50 # probability of successful round
L = 100

# calculated based on the observable Bitcoin hash rate (based on the block production rate of 1 block / 10 minutes)
n = 4340278

simulation = Simulation(f, L, n, q)
print(simulation.simulate_execution()[0].height)