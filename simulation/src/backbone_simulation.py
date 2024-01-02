from .blocktree import Block, balanced_mining, common_prefix_k, heaviest_chain
from numpy.random import binomial

class BackboneSimulation:
  def __init__(self, f, L, n, t, q):
    self.f = f
    self.L = L
    self.n = n
    self.q = q
    self.t = t
    # self.kappa = 256

    # f ~= (n - t) * q * p
    # f = 1 - (1 - p)**(q * (n - t)) # Union bound: q * (n - t) * p
    # solving for p:
    self.p = 1 - (1 - self.f)**(1 / (self.q * (self.n - self.t)))
    # p = T / 2**kappa
    # solving for T:
    # self.T = self.p * 2**self.kappa

    self.genesis = Block()
    self.honest_tips = [self.genesis]
    self.adversarial_tip = self.genesis

  def simulate_execution(self):
    safe_common_prefix = 0

    for r in range(self.L):
      round_result = self.sample_round(r)
      honest_successes = round_result['honest']
      adversarial_successes = round_result['adversarial']

      self.honest_tips = balanced_mining(self.honest_tips, [Block() for _ in range(honest_successes)])
      best_honest_tip = heaviest_chain(self.honest_tips)

      # rushing adversary
      # private mining attack
      # TODO: make this condition parametrizable
      if best_honest_tip.height > self.adversarial_tip.height:
        self.adversarial_tip = best_honest_tip

      for i in range(adversarial_successes):
        block = Block()
        self.adversarial_tip.add_child(block)
        self.adversarial_tip = block

      safe_common_prefix = max(safe_common_prefix, common_prefix_k(self.honest_tips + [self.adversarial_tip]))

    return {
      'tips': self.honest_tips,
      'common_prefix': safe_common_prefix,
    }

  def sample_round(self, r):
    return {
      'honest': binomial(self.n - self.t, 1 - (1 - self.p)**self.q),
      'adversarial': binomial(self.t * self.q, self.p)
    }