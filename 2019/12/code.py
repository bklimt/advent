
# Hard-coded initial data.
x = [-16,   0, -11,  2]
y = [ -1,  -4,  11,  2]
z = [-12, -17,   0, -6]

# Hard-coded first example.
# x = [-1,  2, 4, 3]
# y = [ 0,-10,-8, 5]
# z = [ 2, -7, 8,-1]

dx = [0]*4
dy = [0]*4
dz = [0]*4

# Loop over a fixed number of time steps.
for t in range(1001):
  # Print the current state of the universe.
  print('t=%d' % t)
  for j in range(4):
    print('pos=<x=%d, y=%d, z=%d>, vel=<x=%d, y=%d, z=%d>' %
      (x[j], y[j], z[j], dx[j], dy[j], dz[j]))

  # Update the velocity of each moon.
  for i in range(4):
    for j in range(4):
      dx[i] = dx[i] + cmp(x[j], x[i])
      dy[i] = dy[i] + cmp(y[j], y[i])
      dz[i] = dz[i] + cmp(z[j], z[i])

  # Update the position of each moon and compute energy.
  e = 0
  for i in range(4):
    x[i] = x[i] + dx[i]
    y[i] = y[i] + dy[i]
    z[i] = z[i] + dz[i]
    pot = abs(x[i]) + abs(y[i]) + abs(z[i])
    kin = abs(dx[i]) + abs(dy[i]) + abs(dz[i])
    e = e + (pot * kin)
  print('e=%d' % e)

# 12.1 = 5517
