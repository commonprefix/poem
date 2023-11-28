import numpy as np
from scipy.special import gamma, gammainc, betainc
import matplotlib.pyplot as plt

plt.rcParams['text.usetex'] = True

error = 0.1
DURATION = 1000

def create_gamma_distr(α, β):
  def gamma_cdf(x):
    return β**α * gammainc(α, β * x)

  return gamma_cdf, α / β

def create_binomial_distr(n, p):
  def binomial_cdf(k):
    return betainc(n - k, 1 + k, 1 - p)

  return binomial_cdf, n * p

def get_tail_probability(cdf, mean):
  tailpoint = mean * (1 + error)
  return 1 - cdf(tailpoint)

n = np.linspace(1, DURATION, DURATION)
p_binomial = np.zeros(DURATION)
p_gamma = np.zeros(DURATION)

for i in range(1, DURATION + 1):
  binomial_cdf, binomial_mean = create_binomial_distr(i, 0.5)
  gamma_cdf, gamma_mean = create_gamma_distr(i, np.emath.log(2))
  p_binomial[i - 1] = get_tail_probability(binomial_cdf, binomial_mean)
  p_gamma[i - 1] = get_tail_probability(gamma_cdf, gamma_mean)
  print(p_gamma[i - 1])
plt.plot(n, p_binomial, label='Binomial')
plt.plot(n, p_gamma, label='Gamma')
plt.title(r'Prob of value exceeding $10\%$ of mean')
y = np.exp(-n*(0.01/3))
plt.plot(n, y, label=r'$e^{-\Omega(n)}$')

plt.legend()
plt.show()
