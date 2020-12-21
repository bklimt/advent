
import math

edge_map = {}

def b_to_i(bs):
  return eval('0b' + ''.join([str(b) for b in bs]))

def c_to_b(c):
  if c == '#':
    return 1
  elif c == '.':
    return 0
  else:
    raise Exception('oh no!')

class ArrangedTile:
  def __init__(self, top, bottom, left, right):
    self.top = top
    self.bottom = bottom
    self.left = left
    self.right = right

  def rotateClockwise90(self):
    return ArrangedTile(self.left, self.right, self.bottom, self.top)

  def rotateClockwise270(self):
    return ArrangedTile(self.right, self.left, self.top, self.bottom)

  def rotate180(self):
    return ArrangedTile(self.bottom, self.top, self.right, self.left)

class Tile:
  def __init__(self, id, data):
    self.id = id

    top = b_to_i(data[0])
    top_inv = b_to_i(data[0][::-1])
    bottom = b_to_i(data[-1])
    bottom_inv = b_to_i(data[-1][::-1])
    left = b_to_i([x[0] for x in data])
    left_inv = b_to_i([x[0] for x in data][::-1])
    right = b_to_i([x[-1] for x in data])
    right_inv = b_to_i([x[-1] for x in data][::-1])

    edges = [top, top_inv, bottom, bottom_inv, left, left_inv, right, right_inv]
    for edge in edges:
      if edge not in edge_map:
        edge_map[edge] = [id]
      else:
        if id not in edge_map[edge]:
          edge_map[edge].append(id)

    original = ArrangedTile(top, bottom, left, right)
    flipped_h = ArrangedTile(top_inv, bottom_inv, right, left)
    flipped_v = ArrangedTile(bottom, top, left_inv, right_inv)
    flipped_both = ArrangedTile(bottom_inv, top_inv, right_inv, left_inv)

    self.options = [
      original,
      original.rotateClockwise90(),
      original.rotate180(),
      original.rotateClockwise270(),
      flipped_h,
      flipped_h.rotateClockwise90(),
      flipped_h.rotate180(),
      flipped_h.rotateClockwise270(),
      flipped_v,
      flipped_v.rotateClockwise90(),
      flipped_v.rotate180(),
      flipped_v.rotateClockwise270(),
      flipped_both,
      flipped_both.rotateClockwise90(),
      flipped_both.rotate180(),
      flipped_both.rotateClockwise270(),
    ]

def match_h(t1, t2):
  for opt1 in t1.options:
    for opt2 in t2.options:
      if opt1.right == opt2.left:
        return True
  return False

def match_v(t1, t2):
  for opt1 in t1.options:
    for opt2 in t2.options:
      if opt1.bottom == opt2.top:
        return True
  return False

def parse_tile(s):
  title, data = s.split('\n', 1)
  tile_id = int(title[5:-1])
  data = data.split('\n')
  data = [[c_to_b(c) for c in line] for line in data]
  return Tile(tile_id, data)

def print_board(board):
  for i in range(len(board)):
    for j in range(len(board)):
      print '{0:>3} '.format(len(board[i][j])),
    print

def filter_impossible(board, tile_map, solved):
  changed = False
  for r in range(len(board)):
    for c in range(len(board)):
      # This one is already solved.
      if len(board[r][c]) == 1:
        continue

      new_entry = []
      for t1 in board[r][c]:
        # Is this tile possible?

        # Is this tile already solved?
        if t1 in solved:
          changed = True
          continue

        # Can this tile match one of its neighbors?
        poss_down = False
        if r + 1 < len(board):
          for t2 in board[r+1][c]:
            if match_v(tile_map[t1], tile_map[t2]):
              poss_down = True
              break
        else:
          poss_down = True

        poss_up = False
        if r > 0:
          for t2 in board[r-1][c]:
            if match_v(tile_map[t2], tile_map[t1]):
              poss_up = True
              break
        else:
          poss_up = True

        poss_right = False
        if c + 1 < len(board):
          for t2 in board[r][c+1]:
            if match_h(tile_map[t1], tile_map[t2]):
              poss_right = True
              break
        else:
          poss_right = True

        poss_left = False
        if c > 0:
          for t2 in board[r][c-1]:
            if match_h(tile_map[t2], tile_map[t1]):
              poss_left = True
              break
        else:
          poss_left = True

        if poss_up and poss_down and poss_left and poss_right:
          new_entry.append(t1)
        else:
          changed = True

      board[r][c] = new_entry
      if len(new_entry) == 1:
        solved[new_entry[0]] = True

  # The whole thing could be flipped diagonally, so pick one.
  if len(board[0][1]) == 2:
    picked = board[0][1][0]
    board[0][1] = [picked]
    solved[picked] = True
    changed = True

  return changed

def find_corners(tiles):
  corners = []
  for tile in tiles:
    for option in tile.options:
      if len(edge_map[option.left]) == 1:
        if len(edge_map[option.top]) == 1:
          corners.append(tile.id)
          break
  return corners

def matrix_set(m, r, c, d):
  while len(m) < r:
    m.append([])
  while len(m[r]) < c:
    m[r].append(' ')
  m[r][c] = d

def create_image(board, tile_map):
  dim = len(board) * 8
  image = []
  for r in range(len(board)):
    for c in range(len(board)):
      if len(board[r][c]) != 1:
        raise Exception("oh no!")
      tile = tile_map[board[r][c][0]]
      # TODO(klimt): Oh no! I don't know the orientation!
      option = tile.options[0]
      for rr in range(0, 7):
        for cc in range(0, 7):
          matrix_set(image, r*8+rr, c*8+cc, option.data[rr+1][cc+1])

def main():
  tile_strs = file('input.txt').read().strip().split('\n\n')
  tiles = []
  tile_map = {}
  for s in tile_strs:
    tile = parse_tile(s)
    tiles.append(tile)
    tile_map[tile.id] = tile
  print len(tiles)
  dim = int(math.sqrt(len(tiles)))
  print '%d x %d' % (dim, dim)

  board = []
  for i in range(dim):
    row = []
    for j in range(dim):
      t = []
      for tile in tiles:
        t.append(tile.id)
      row.append(t)
    board.append(row)

  print_board(board)
  print

  corners = find_corners(tiles)
  print corners
  if len(corners) != 4:
    raise Exception("oh no!")

  # 16192267830719
  print 'Part 1:', corners[0] * corners[1] * corners[2] * corners[3]
  print

  # Fix one corner and make the others be corners.
  board[0][0] = [corners[0]]
  board[0][-1] = [corners[1], corners[2], corners[3]]
  board[-1][0] = [corners[1], corners[2], corners[3]]
  board[-1][-1] = [corners[1], corners[2], corners[3]]

  solved = {corners[0]: True}

  changed = True
  while changed:
    changed = filter_impossible(board, tile_map, solved)
    print_board(board)
    print

main()
