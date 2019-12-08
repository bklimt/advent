
def booltostr(b):
  if b:
    return 'X'
  else:
    return ' '

def printadj(adj):
  print('\n'.join([''.join([booltostr(col) for col in row]) for row in adj]))

def count(adj):
  t = 0
  for i in range(len(adj)):
    for j in range(len(adj)):
      if adj[i][j]:
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
  adj = [[False for i in range(n)] for j in range(n)]
  # Do the immediate orbits.
  for x in xs:
    a, b = x
    adj[index[a]][index[b]] = True
  printadj(adj)
  # Compute the transitive orbits.
  for h in range(n):
    print('%d/%d' % (h, n))
    for i in range(n):
      for j in range(n):
        if not adj[i][j]:
          for k in range(n):
            if adj[i][k] and adj[k][j]:
              adj[i][j] = True
  printadj(adj)
  print('checksum = %d' % count(adj))

main()
