
lines = file('input.txt').read().strip().split()
tiles = {}

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

def main():
  for line in lines:
    x, y = follow(line)
    if (x, y) not in tiles:
      tiles[(x, y)] = True
    else:
      tiles[(x, y)] = not tiles[(x, y)]

  count = 0
  for k in tiles:
    if tiles[k]:
      count = count + 1

  print count

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