from math import log, e
from scipy.optimize import newton
from numpy import arange

def get_funcs(g):
  def f(α):
    return α * 2**α - g / log(2, e)

  def f_prime(α):
    return 2**α + α * log(2, e) * 2**α
  
  return f, f_prime

for g in arange(0, 10, 0.1):
  f, f_prime = get_funcs(g)
  α = newton(f, g, f_prime)
  # adv_work = g / log(2, e) * β / (1 - β)
  # hon_work = α
  bitcoin_β = 1 / (g + 2)
  poem_β = α * log(2, e) / (g + α * log(2, e))
  print('g = {:.2f}'.format(g))
  print(
     '\tBitcoin:\tadv rate = {:.2f}, hon rate = {:.2f}, β = {:.2f}'.format(
        g * bitcoin_β / (1 - bitcoin_β), g / (g + 1), bitcoin_β
      )
  )
  print(
    '\tPoEM:\tadv rate = {:.2f}, hon rate = {:.2f}, β = {:.2f}'.format(
       (g * poem_β / (1 - poem_β)) / log(2, e), α, poem_β
    )
  )