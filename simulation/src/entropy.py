from math import log
import numpy as np

kappa = 256

def poem_get_work():
  return kappa - log(np.random.uniform(1, 2**kappa), 2)

s = 0
N = 10000000
for _ in range(N):
  s += poem_get_work()

print(s/N)
