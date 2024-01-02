import unittest
from numpy import random
from src.backbone_simulation import BackboneSimulation

class TestSimulation(unittest.TestCase):
  def test_simulation(self):
    random.seed(0)

    q = 1
    f = 1/5 # probability of successful round
    L = 100000
    n = 1000
    t = 200

    simulation = BackboneSimulation(f, L, n, t, q)
    honest_successes = 0
    adversarial_successes = 0
    total_trials = 100
    for _ in range(total_trials):
      simulation_result = simulation.sample_round(0)
      honest_successes += simulation_result['honest']
      adversarial_successes += simulation_result['adversarial']

    self.assertAlmostEqual(adversarial_successes / (honest_successes + adversarial_successes), t/n, delta=0.1)
