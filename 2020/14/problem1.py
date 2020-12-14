
lines = file('sample.txt').read().strip().split('\n')
mask = lines[0][7:]
memory = {}

def mask_char(c1, c2, n1, n2):
  if c1 == c2:
    return n1
  return n2

mask0 = eval('0b' + ''.join([mask_char(x, '0', '0', '1') for x in mask]))
mask1 = eval('0b' + ''.join([mask_char(x, '1', '1', '0') for x in mask]))

def apply_mask(n):
  n = n & mask0
  n = n | mask1
  return n

class Command:
  def __init__(self, addr, val):
    self.addr = addr
    self.val = val
  def run(self):
    memory[self.addr] = apply_mask(self.val)
  def __str__(self):
    return 'mem[' + str(self.addr) + '] = ' + str(self.val)
  def __repr__(self):
    return str(self)

def parse_command(s):
  lhs, rhs = s.split(' = ')
  addr = int(lhs[4:-1])
  val = int(rhs)
  return Command(addr, val)

cmds = [parse_command(line) for line in lines[1:]]

print mask
print mask0
print mask1
print cmds
  
for cmd in cmds:
  print memory
  print cmd
  cmd.run()

print memory

sum = 0
for k in memory:
  sum = sum + memory[k]
print sum
