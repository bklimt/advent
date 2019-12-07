
# Returns a list of line segments.
def segs(w):
  print(w)
  s = []
  p1 = (0, 0)
  p2 = None
  for c in w:
    # print(c)
    if c[0] == 'R':
      p2 = (p1[0] + int(c[1:]), p1[1])
    if c[0] == 'L':
      p2 = (p1[0] - int(c[1:]), p1[1])
    if c[0] == 'D':
      p2 = (p1[0], p1[1] + int(c[1:]))
    if c[0] == 'U':
      p2 = (p1[0], p1[1] - int(c[1:]))
    # print(p2)
    s.append((p1, p2))
    p1 = p2
    p2 = None
  return s

def mlen(s):
  return abs(s[1][1] - s[0][1]) + abs(s[1][0] - s[0][0])

# Returns the point where two segments intersect.
def intersect(s1, s2):
  # TODO
  # There are 3 cases:
  # 1. Both horizontal.
  # 2. Both vertical.
  # 3. A horizontal and a vertical.
  #
  return None

def find(s1, s2):
  min = None
  for i in range(len(s1)):
    for j in range(len(s2)):
      p = intersect(s1[i], s2[j])
      if p is not None:
        if min is None:
          min = p
        else:
          if mlen(p) < mlen(min):
            min = p
  print('min: %s' % (repr(min)))
  if min is None:
    return -1
  return mlen(min)

# Finds the closest intersection.
def run(w1, w2):
  s1 = segs(w1.split(','))
  s2 = segs(w2.split(','))
  print(s1)
  print(s2)
  find(s1, s2)
  return 0

def main():
  print(run("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"))

main()
