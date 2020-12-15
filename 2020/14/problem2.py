
lines = file('input.txt').read().strip().split('\n')

class State:
  def __init__(self):
    self.mask = None
    self.memory = {}

  def parse_mask(self, s):
    self.mask = s[7:]
    print 'mask = ' + self.mask

  def apply_mask(self, base_addr):
    addrs = ['']
    for i in range(36):
      new_addrs = []
      for addr in addrs:
        if self.mask[i] == '1':
          new_addrs.append(addr + '1')
        elif self.mask[i] == '0':
          new_addrs.append(addr + base_addr[i])
        elif self.mask[i] == 'X':
          new_addrs.append(addr + '0')
          new_addrs.append(addr + '1')
        else:
          raise Exception('oh no!')
      addrs = new_addrs
    print base_addr + ' -> ' + repr(addrs)
    return addrs

  def run(self, cmd):
    for addr in self.apply_mask(cmd.addr):
      self.memory[addr] = cmd.val

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
  addr = format(int(lhs[4:-1]), '0>36b')
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
