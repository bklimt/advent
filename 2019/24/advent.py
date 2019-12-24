
def boolstr(b):
  if b:
    return '#'
  else:
    return '.'

def boardstr(b):
  return '\n'.join([''.join([boolstr(c) for c in row]) for row in b])

def printboard(board):
  print(boardstr(board))

def gen(b):
  b2 = [[False]*5 for _ in range(5)]
  for i in range(5):
    for j in range(5):
      count = 0
      if i > 0 and b[i-1][j]:
        count = count + 1
      if i < 4 and b[i+1][j]:
        count = count + 1
      if j > 0 and b[i][j-1]:
        count = count + 1
      if j < 4 and b[i][j+1]:
        count = count + 1
      if b[i][j]:
        b2[i][j] = (count == 1)
      else:
        b2[i][j] = (count == 1 or count == 2)
  return b2

def main():
  seen = {}
  with open('input.txt') as f:
    s = f.read()
    board = [[c == '#' for c in line] for line in s.split('\n')]
    while boardstr(board) not in seen:
      seen[boardstr(board)] = True
      board = gen(board)
      printboard(board)
      print('')

main()

# #####
# .....
# ..#.#
# ....#
# #...#
#
# 0b1000110000101000000011111
#