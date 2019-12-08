
def booltostr(b):
  if b is not None:
    return 'X'
  else:
    return ' '

def inttostr(b):
  if b is not None:
    return str(b)
  else:
    return ' '

def printadj(adj):
  print('\n'.join([''.join([booltostr(col) for col in row]) for row in adj]))

def count(adj):
  t = 0
  for i in range(len(adj)):
    for j in range(len(adj)):
      if adj[i][j] is not None:
        t = t + 1
  return t

def main():
  f = open('input.txt')
  s = f.read()
  f.close()
  ss = s.split()
  xs = [x.split(')') for x in ss]
  index = {}
  for x in xs:
    a, b = x
    print('%s -> %s' % (a, b))
    if a not in index:
      n = len(index)
      index[a] = n
    if b not in index:
      n = len(index)
      index[b] = n
  print(index)
  n = len(index)
  adj = [[None for i in range(n)] for j in range(n)]
  # Do the immediate orbits.
  for x in xs:
    a, b = x
    adj[index[a]][index[b]] = 1
    adj[index[b]][index[a]] = 1
  printadj(adj)
  # Compute the transitive orbits.
  for h in range(n):
    mod = 0
    print('%d/%d' % (h, n))
    for i in range(n):
      for j in range(n):
        for k in range(n):
          ij = adj[i][j]
          ik = adj[i][k]
          kj = adj[k][j]
          if ik is not None and kj is not None:
            if ij is None or ik + kj < ij:
              mod = mod + 1
              adj[i][j] = ik + kj
    if mod == 0:
      break
    print('modified: %d' % mod)
  # printadj(adj)
  # print('checksum = %d' % count(adj))
  print('distance = %d' % adj[index['YOU']][index['SAN']]-2)

main()

# 1: 147223
# 2: 340
