
lines = file('input.txt').read().strip().split()

def follow(line):
  x = 0
  y = 0
  i = 0
  while len(line):
    if line[0] == 'n' or line[0] == 's':
      cmd, line = line[:2], line[2:]
    else:
      cmd, line = line[:1], line[1:]
    if cmd == 'ne':
      x = x + 1
      y = y - 1
    elif cmd == 'nw':
      x = x - 1
      y = y - 1
    elif cmd == 'se':
      x = x + 1
      y = y + 1
    elif cmd == 'sw':
      x = x - 1
      y = y + 1
    elif cmd == 'e':
      x = x + 2
    elif cmd == 'w':
      x = x - 2
  return x, y

class Board:
  def __init__(self):
    self.tiles = {}
    self.minX = 0
    self.minY = 0
    self.maxX = 0
    self.maxY = 0

  def set(self, x, y, v):
    if x < self.minX:
      self.minX = x
    if y < self.minY:
      self.minY = y
    if x >= self.maxX:
      self.maxX = x + 1
    if y >= self.maxY:
      self.maxY = y + 1
    if v:
      self.tiles[(x, y)] = True
    elif (x, y) in self.tiles:
      del self.tiles[(x, y)]

  def toggle(self, x, y):
    self.set(x, y, not self.check(x, y))

  def count(self):
    count = 0
    for k in self.tiles:
      if self.tiles[k]:
        count = count + 1
    return count

  def check(self, x, y):
    return ((x, y) in self.tiles) and self.tiles[(x, y)]

  def cycle(self):
    b = Board()
    for x in range(self.minX - 2, self.maxX + 2):
      for y in range(self.minY - 1, self.maxY + 1):
        neighbor_xy = [
          (x-2, y),
          (x+2, y),
          (x-1, y-1),
          (x-1, y+1),
          (x+1, y-1),
          (x+1, y+1),
        ]
        neighbors = [self.check(c[0], c[1]) for c in neighbor_xy]
        on_neighbors = len([n for n in neighbors if n])

        if self.check(x, y):
          if on_neighbors == 1 or on_neighbors == 2:
            b.set(x, y, True)
        else:
          if on_neighbors == 2:
            b.set(x, y, True)
    return b

def main():
  board = Board()
  for line in lines:
    x, y = follow(line)
    board.toggle(x, y)
  print board.count()

  for i in range(100):
    board = board.cycle()
    print (i+1, board.count())

# print follow('nwwswee')

main()

#   0 1 2 3 4 5 6
# 0 |   |   |   |
#    \ / \ / \ /
# 1   | 3 | 2 |
#    / \ / \ / \
# 2 | 4 | 5 | 1 |
#    \ / \ / \ /
# 3   |   |   |
#    / \ / \ / \
# 4 |   |   |   |
#
# (2,5) - nw - (1,4) - w - (1,2) - sw - (2,1) - e - (2,3) - e - (2,5)