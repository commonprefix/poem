from numpy import linspace, geomspace

seconds_of_simulation = 0
honest_simulation_count = 1800
adv_simulation_count = 600

Δ = 12 # seconds

for g in geomspace(0.01, 100, 70):
  for β in linspace(0.01, 0.45, 33):
    num_γ = 40

    adv_simulation_lifetime = adv_simulation_count / (g * β / (1 - β))
    honest_simulation_lifetime = honest_simulation_count / g

    seconds_of_simulation += num_γ * (adv_simulation_lifetime * Δ + honest_simulation_lifetime * Δ)

years_of_simulation = seconds_of_simulation / (365 * 24 * 60 * 60)
print(years_of_simulation)