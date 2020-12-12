
cmds = file('input.txt').read().strip().split()

wx = 10
wy = -1
x = 0
y = 0

for cmd in cmds:
  amt = int(cmd[1:])
  if cmd[0] == 'N':
    wy = wy - amt
  elif cmd[0] == 'S':
    wy = wy + amt
  elif cmd[0] == 'W':
    wx = wx - amt
  elif cmd[0] == 'E':
    wx = wx + amt
  elif cmd[0] == 'F':
    x = x + wx * amt
    y = y + wy * amt
  elif cmd[0] == 'R' or cmd[0] == 'L':
    if cmd[0] == 'L':
      amt = 360 - amt
    if amt == 90:
      wx, wy = wy * -1, wx
    elif amt == 270:
      wx, wy = wy, wx * -1
    elif amt == 180:
      wx, wy = wx * -1, wy * -1
    else:
      raise Exception('deg: ' + str(amt))
  else:
    raise Exception('???: ' + cmd)
  print 'cmd: %s, amt: %d, pos: (%d, %d), w: (%d, %d)' % (cmd, amt, x, y, wx, wy)

print abs(x) + abs(y)