
cmds = file('input.txt').read().strip().split()

x = 0
y = 0
dx = 1
dy = 0

for cmd in cmds:
  amt = int(cmd[1:])
  if cmd[0] == 'N':
    y = y - amt
  elif cmd[0] == 'S':
    y = y + amt
  elif cmd[0] == 'W':
    x = x - amt
  elif cmd[0] == 'E':
    x = x + amt
  elif cmd[0] == 'F':
    x = x + dx * amt
    y = y + dy * amt
  elif cmd[0] == 'R' or cmd[0] == 'L':
    if cmd[0] == 'L':
      amt = 360 - amt
    if amt == 90:
      if dx == 0:
        dx, dy = dy * -1, 0
      else:
        dx, dy = 0, dx
    elif amt == 270:
      if dx == 0:
        dx, dy = dy, 0
      else:
        dx, dy = 0, dx * -1
    elif amt == 180:
      dx = dx * -1
      dy = dy * -1
    else:
      raise Exception('deg: ' + str(amt))
  else:
    raise Exception('???: ' + cmd)
  print 'cmd: %s, amt: %d, pos: (%d, %d), dir: (%d, %d)' % (cmd, amt, x, y, dx, dy)

print abs(x) + abs(y)