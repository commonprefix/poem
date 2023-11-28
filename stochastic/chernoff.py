import numpy as np
from scipy.special import gamma, gammainc, betainc
import matplotlib.pyplot as plt

plt.rcParams['text.usetex'] = True

error = 0.1
DURATION = 1000

def create_gamma_distr(α, β):
  def gamma_cdf(x):
    return gammainc(α, β * x)

  return gamma_cdf, α / β

def create_binomial_distr(n, p):
  def binomial_cdf(k):
    return betainc(n - k, 1 + k, 1 - p)

  return binomial_cdf, n * p

def create_biased_gamma_distr(α, β, γ):
  def gamma_cdf(x):
    return gammainc(α, β * x)

  def biased_gamma_cdf(x):
    print(gamma_cdf(max(x - α * γ, 0)))
    return gamma_cdf(max(x - α * γ, 0))

  return biased_gamma_cdf, γ * α + α / β

def get_tail_probability(cdf, mean):
  tailpoint = mean * (1 + error)
  return 1 - cdf(tailpoint)

n = np.linspace(1, DURATION, DURATION)
p_binomial = np.zeros(DURATION)
p_gamma = np.zeros(DURATION)
p_biased_gamma = np.zeros(DURATION)

γ = 10
λ = 1.1

for i in range(1, DURATION + 1):
  binomial_cdf, binomial_mean = create_binomial_distr(i, 0.5)
  gamma_cdf, gamma_mean = create_gamma_distr(i, λ)
  biased_gamma_cdf, biased_gamma_mean = create_biased_gamma_distr(i, λ, γ)
  p_binomial[i - 1] = get_tail_probability(binomial_cdf, binomial_mean)
  p_gamma[i - 1] = get_tail_probability(gamma_cdf, gamma_mean)
  p_biased_gamma[i - 1] = get_tail_probability(biased_gamma_cdf, biased_gamma_mean)
plt.plot(n, p_binomial, label='Binomial')
plt.plot(n, p_gamma, label='Gamma')
plt.plot(n, p_biased_gamma, label=r'Biased Gamma $\gamma = {}$'.format(γ))
plt.title(r'Prob of value exceeding $10\%$ of mean')
y = np.exp(-n*(0.01/3))
plt.plot(n, y, label=r'$e^{-\Omega(n)}$')

plt.legend()
plt.show()
