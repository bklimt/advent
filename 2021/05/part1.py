
def sign(n):
  if n > 0:
    return 1
  if n < 0:
    return -1
  return 0

def direction(n1, n2):
  return sign(n2 - n1)

counts = {}

f = open('input.txt')
for line in f:
  left, right = line.split(' -> ')
  x1, y1 = left.split(',')
  x2, y2 = right.split(',')
  x1, y1, x2, y2 = int(x1), int(y1), int(x2), int(y2)
  if x1 != x2 and y1 != y2:
    continue
  dx = direction(x1, x2)
  dy = direction(y1, y2)
  x = x1
  y = y1
  x2 = x2 + dx
  y2 = y2 + dy
  while x != x2 or y != y2:
    p = (x, y)
    if p not in counts:
      counts[p] = 0
    counts[p] = counts[p] + 1
    x = x + dx
    y = y + dy

ans = 0
for p in counts:
  if counts[p] > 1:
    print(p)
    ans = ans + 1
print(ans)