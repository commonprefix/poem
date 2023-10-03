import unittest
from blocktree import Block, common_prefix_k, LCA

class TestBlockTree(unittest.TestCase):
  def test_height(self):
    genesis = Block()
    self.assertEqual(genesis.height, 0)

  def test_lca(self):
    genesis = Block()

    j1, j2 = LCA(genesis, genesis)

    self.assertEqual((j1, j2), (0, 0))

    Ba = genesis.extend()
    Bb = genesis.extend()

    j1, j2 = LCA(Ba, Bb)
    self.assertEqual((j1, j2), (1, 1))

    Ba1 = Ba.extend()
    j1, j2 = LCA(Ba1, Bb)
    self.assertEqual((j1, j2), (2, 1))

    j1, j2 = LCA(Bb, Ba1)
    self.assertEqual((j1, j2), (1, 2))

  def test_common_prefix(self):
    genesis = Block()
    self.assertEqual(common_prefix_k([genesis]), 0)
    self.assertEqual(common_prefix_k([genesis, genesis]), 0)

    Ba = genesis.extend()
    Bb = genesis.extend()

    self.assertEqual(common_prefix_k([Ba, Bb]), 1)

    Ba1 = Ba.extend()

    self.assertEqual(common_prefix_k([Ba1, Bb]), 2)

  def test_leaves(self):
    genesis = Block()

    Ba = genesis.extend()
    Bb = genesis.extend()

    leaves = list(genesis.leaves())

    self.assertEqual(len(leaves), 2)
    self.assertEqual(leaves[0], Ba)
    self.assertEqual(leaves[1], Bb)

    Ba1 = Ba.extend()

    leaves = list(genesis.leaves())

    self.assertEqual(len(leaves), 2)