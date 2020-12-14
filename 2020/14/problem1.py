
lines = file('input.txt').read().strip().split('\n')

def mask_char(c1, c2, n1, n2):
  if c1 == c2:
    return n1
  return n2

class State:
  def __init__(self):
    self.mask = 0
    self.memory = {}
    self.mask0 = 0
    self.mask1 = 0

  def parse_mask(self, s):
    self.mask = s[7:]
    self.mask0 = eval('0b' + ''.join([mask_char(x, '0', '0', '1') for x in self.mask]))
    self.mask1 = eval('0b' + ''.join([mask_char(x, '1', '1', '0') for x in self.mask]))

  def apply_mask(self, n):
    n = n & self.mask0
    n = n | self.mask1
    return n

  def run(self, cmd):
    self.memory[cmd.addr] = self.apply_mask(cmd.val)

class Command:
  def __init__(self, addr, val):
    self.addr = addr
    self.val = val
  def __str__(self):
    return 'mem[' + str(self.addr) + '] = ' + str(self.val)
  def __repr__(self):
    return str(self)

def parse_command(s):
  lhs, rhs = s.split(' = ')
  addr = int(lhs[4:-1])
  val = int(rhs)
  return Command(addr, val)

def main():
  state = State()
  for line in lines:
    if line[1] == 'a':
      state.parse_mask(line)
    else:
      cmd = parse_command(line)
      state.run(cmd)

  print state.memory

  sum = 0
  for k in state.memory:
    sum = sum + state.memory[k]
  print sum

main()
