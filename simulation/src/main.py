from .backbone_simulation import BackboneSimulation

kappa = 256
hash_rate = 1
q = 1
f = 1/5 # probability of successful round
L = 100000
n = 1000
t = 200

simulation = BackboneSimulation(f, L, n, t, q)
execution = simulation.simulate_execution()
tips = execution['tips']
common_prefix = execution['common_prefix']

print('Number of tips:', len(tips))
print('Number of leaves:', len(list(simulation.genesis.leaves())))
print('Height of the first chain:', tips[0].height)
print('Minimum safe common prefix:', common_prefix)
round_duration = 12 # seconds
print('Confirmation time:', common_prefix * round_duration / f, 'seconds')