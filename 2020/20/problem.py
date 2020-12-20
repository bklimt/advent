
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

def parse_tile(s):
  title, data = s.split('\n', 1)
  tile_id = int(title[5:-1])
  data = data.split('\n')
  data = [[c_to_b(c) for c in line] for line in data]
  edges = [
    b_to_i(data[0]),
    b_to_i(data[0][::-1]),
    b_to_i(data[-1]),
    b_to_i(data[-1][::-1]),
    b_to_i([x[0] for x in data]),
    b_to_i([x[0] for x in data][::-1]),
    b_to_i([x[-1] for x in data]),
    b_to_i([x[-1] for x in data][::-1]),
  ]
  for edge in edges:
    if edge not in edge_map:
      edge_map[edge] = [tile_id]
    else:
      if tile_id not in edge_map[edge]:
        edge_map[edge].append(tile_id)

def find_unmatched():
  for edge in edge_map:
    if len(edge_map[edge]) == 1:
      print edge, edge_map[edge]

def main():
  tile_strs = file('input.txt').read().strip().split('\n\n')
  for s in tile_strs:
    parse_tile(s)

  find_unmatched()

main()
