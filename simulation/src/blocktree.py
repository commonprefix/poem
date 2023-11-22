from itertools import combinations, cycle
from math import log2

kappa = 256

def LCA(C1, C2):
  if C2.height < C1.height:
    j2, j1 = LCA(C2, C1)
    return j1, j2

  # C2 is the longer chain
  B1, B2 = C1, C2
  while B1 != B2:
    if B1.height == B2.height:
      B1 = B1.parent
    B2 = B2.parent
  return C1.height - B1.height, C2.height - B2.height

'''
Returns the smallest k such that Common Prefix between these adopted tips holds.
'''
# TODO: Use heavy–light decomposition to optimize this
def common_prefix_k(tips):
  cp = 0

  for C1, C2 in combinations(tips, 2):
    j1, j2 = LCA(C1, C2)
    cp = max(cp, max(j1, j2))

  return cp

def heaviest_chain(tips):
  return max(tips, key=lambda tip: tip.chain_work)

def balanced_mining(tips, blocks):
  for block, tip in zip(blocks, cycle(tips)):
    tip.add_child(block)

  if blocks:
    return blocks
  return tips

class Work:
  @staticmethod
  def nakamoto(hash):
    return 1

  @staticmethod
  def poem(hash):
    return kappa - log2(hash)

class Block:
  def __init__(self, hash=None, block_work_function=Work.nakamoto):
    self.children = []
    self.parent = None
    self.height = 0
    self.hash = hash
    self.block_work_function = block_work_function
    self.block_work = block_work_function(hash)
    self.chain_work = self.block_work

  def add_child(self, child):
    self.children.append(child)
    child.parent = self
    child.height = self.height + 1
    child.chain_work = self.chain_work + child.block_work

  def extend(self, *args, **kwargs):
    child = Block(*args, **kwargs)
    self.add_child(child)
    return child

  def depth(self):
    if not self.children:
      return 0

    child_depth = 0
    for child in self.children:
      child_depth = max(child_depth, child.depth() + 1)

    return child_depth

  def bfs(self):
    q = [self]

    while q:
      node = q.pop(0)
      yield node

      for child in node.children:
        q.append(child)

  def leaves(self):
    yield from (node for node in self.bfs() if not node.children)

  def root(self):
    if self.parent is None:
      return self

    return self.parent.root()