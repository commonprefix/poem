from math import log, e
import numpy as np

g = 1

def limit(α):
  for dt in np.arange(0.0001, 0.00001, -0.00001):
    num = α * dt * log(2, e)
    denom = 2**(α * (dt - 1)) * (1 - e**(-g * dt))
    print(num / denom)

limit(0.8182157)