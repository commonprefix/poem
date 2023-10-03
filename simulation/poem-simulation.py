from blocktree import Block, common_prefix_k
from numpy.random import binomial, randint
from math import log

class PoEMSimulation:
  def __init__(self, backbone_f, L, n, q):
    self.kappa = 256

    self.backbone_f = backbone_f
    self.L = L
    self.n = n
    self.q = q
    self.t = 0

    self.p = 1 - (1 - self.backbone_f)**(1 / (self.q * (self.n - self.t)))
    self.T = self.p * 2**self.kappa

    self.genesis = Block()
    self.tips = [self.genesis]

  def simulate_execution(self):
    safe_common_prefix = 0

    for r in range(self.L):
      works = self.simulate_round(r)

      # balancing attack
      new_tips = []
      for tip in self.tips:
        for i in range(len(works) // len(self.tips)):
          block = Block()
          tip.add_child(block)
          new_tips.append(block)
      for i in range(len(works) % len(self.tips)):
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
    successes = binomial(self.n - self.t, 1 - (1 - self.p)**self.q)

    works = []

    for _ in range(successes):
      # Unsure if these two random variables are "mostly" a translation of each other:
      # X ~ U(1, T)
      # Y ~ U(1, 2^kappa)
      # (Y - T) / T ~= X / 2^kappa (in distribution)
      # log((Y - T) / T) ~= log(X / 2^kappa)
      # log(Y - T) - log(T) ~= log(X) - kappa
      # kappa - log(Y - T) + log(T) ~= kappa - log(X) + kappa
      # kappa - log(X) ~= kappa - log(Y) + log(T) (in distribution)?

      works.append(self.kappa - log(randint(1, self.T), 2))

    return works