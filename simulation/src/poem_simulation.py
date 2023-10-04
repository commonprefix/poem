from blocktree import Block, balanced_mining, common_prefix_k
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
      works = self.sample_round(r)

      self.honest_tips = balanced_mining(self.honest_tips, len(works))

      # TODO: Use heavy–light decomposition to optimize this
      safe_common_prefix = max(safe_common_prefix, common_prefix_k(self.tips))

    return {
      'tips': self.tips,
      'common_prefix': safe_common_prefix,
    }

  def sample_round(self, r):
    honest_successes = binomial(self.n - self.t, 1 - (1 - self.p)**self.q)
    adversarial_success = binomial(self.t * self.q, self.p)

    # Unsure if these two random variables are "mostly" a translation of each other:
    # X ~ U(1, T)
    # Y ~ U(1, 2^kappa)
    # (Y - T) / T ~= X / 2^kappa (in distribution)
    # log((Y - T) / T) ~= log(X / 2^kappa)
    # log(Y - T) - log(T) ~= log(X) - kappa
    # kappa - log(Y - T) + log(T) ~= kappa - log(X) + kappa
    # kappa - log(X) ~= kappa - log(Y) + log(T) (in distribution)?

    return {
      'honest': [randint(1, self.T) for _ in range(honest_successes)],
      'adversarial': [randint(1, self.T) for _ in range(adversarial_success)]
    }