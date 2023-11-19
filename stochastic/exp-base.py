from math import log, e
import numpy as np

NUM_SAMPLES = 3 # 30
MONTE_CARLO = 1000

for λ in np.arange(0.1, 100, 0.1):
  honest_work = 0
  adv_work = 0
  for _ in range(MONTE_CARLO):
    works = []
    for samples in range(NUM_SAMPLES):
      # λ = log(2, e)
      works.append(1 + np.random.exponential(1/λ))
      # works.append(1)
      # works.append(np.random.exponential(1/λ))
    adv_work += sum(works)
    honest_work += max(works)
    
  adv_work /= MONTE_CARLO
  honest_work /= MONTE_CARLO

  print('λ = {:.2f}; Res: {:.2f}'.format(λ, adv_work / honest_work))