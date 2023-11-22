# Discrete time is the wrong model here.

import numpy as np
from math import log, e

MONTE_CARLO = 1000
avg_honest_growth = 0
for _ in range(MONTE_CARLO):
  g = 1
  k = np.random.poisson(g)
  honest_growth = 0
  for _ in range(k):
    work = 1 # np.random.exponential(log(2, e))
    honest_growth = max(honest_growth, work)
  avg_honest_growth += honest_growth
avg_honest_growth /= MONTE_CARLO
print('g = {:.2f}, α = {:.2f}'.format(g, avg_honest_growth))