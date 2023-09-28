from hashlib import sha256
from random import randint

def H(x):
  return int(sha256(x.encode()).digest().hex(), 16)

kappa = 256
round_duration = 12 # seconds
tera = 10**12
hash_rate = 300 * tera # per second in SHA256
q = hash_rate * round_duration # number of queries per round
n = 4340278 # calculated based on the observable Bitcoin hash rate

def simulate(f, L, t, n, q):
  # f = 1 - (1 - p)**(q * (n - t)) # Union bound: q * (n - t) * p
  # solving for p:
  p = 1 - (1 - f)**(1 / (q * (n - t)))
  # p = T / 2**kappa
  # solving for T:
  T = p * 2**kappa
  for r in range(L): # round
    for i in range(n - t): # honest party
      ctr = randint(0, 2**kappa)
      for j in range(q):
        if H(str(ctr)) < T:
          success += 1
        ctr += 1