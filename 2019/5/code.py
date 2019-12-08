import sys

class Interactive:
  def input(self):
    print('input: ')
    return input()
  def output(self, d1):
    print('output: %d' % d1)

class Computer:
  def __init__(self, memory, io):
    self.io = io
    self.running = True
    self.memory = memory
    self.ip = 0
    self.modes = 0

  def log(self, s):
    print(s)

  def quit(self):
    self.log("exiting")
    self.running = False

  def add(self, d1, d2, a3):
    d3 = d1 + d2
    self.memory[a3] = d3
    self.log("[%d] = %d + %d = %d" % (a3, d1, d2, d3))

  def multiply(self, d1, d2, a3):
    d3 = d1 * d2
    self.memory[a3] = d3
    self.log("[%d] = %d * %d = %d" % (a3, d1, d2, d3))

  def input(self, a1):
    d1 = self.io.input()
    self.memory[a1] = d1
    self.log("[%d] = %d" % (a1, d1))

  def output(self, d1):
    self.io.output(d1)

  def jumpif(self, d1, d2):
    if d1 != 0:
      self.ip = d2

  def jumpifnot(self, d1, d2):
    if d1 == 0:
      self.ip = d2

  def lessthan(self, d1, d2, a3):
    if d1 < d2:
      self.memory[a3] = 1
    else:
      self.memory[a3] = 0

  def equals(self, d1, d2, a3):
    if d1 == d2:
      self.memory[a3] = 1
    else:
      self.memory[a3] = 0

  def fetch(self):
    d1 = self.memory[self.ip]
    self.ip = self.ip + 1
    return d1

  def fetchop(self):
    op = self.fetch()
    opcode = op % 100
    op = op / 100
    self.modes = op
    return opcode

  def fetchdata(self):
    mode = self.modes % 10
    self.modes = self.modes / 10
    d1 = self.fetch()
    if mode == 0:
      return self.memory[d1]
    return d1

  def fetchaddr(self):
    mode = self.modes % 10
    self.modes = self.modes / 10
    d1 = self.fetch()
    return d1

  def process(self):
    opcode = self.fetchop()
    if opcode == 99:
      self.quit()
    elif opcode == 1:
      a = self.fetchdata()
      b = self.fetchdata()
      c = self.fetchaddr()
      self.add(a, b, c)
    elif opcode == 2:
      a = self.fetchdata()
      b = self.fetchdata()
      c = self.fetchaddr()
      self.multiply(a, b, c)
    elif opcode == 3:
      a = self.fetchaddr()
      self.input(a)
    elif opcode == 4:
      a = self.fetchdata()
      self.output(a)
    elif opcode == 5:
      a = self.fetchdata()
      b = self.fetchdata()
      self.jumpif(a, b)
    elif opcode == 6:
      a = self.fetchdata()
      b = self.fetchdata()
      self.jumpifnot(a, b)
    elif opcode == 7:
      a = self.fetchdata()
      b = self.fetchdata()
      c = self.fetchaddr()
      self.lessthan(a, b, c)
    elif opcode == 8:
      a = self.fetchdata()
      b = self.fetchdata()
      c = self.fetchaddr()
      self.equals(a, b, c)
    else:
      self.log("unknown opcode: %d" % opcode)
      self.quit()

  def run(self):
    self.log("running %s" % self.memory)
    self.running = True
    self.ip = 0
    while self.running:
      self.process()
      # self.log('state: %s' % self.memory)
    self.log("halted.")
    return self.memory[0]

def run(m):
  print('*' * 80)
  comp = Computer(m, Interactive())
  val = comp.run()
  print('*' * 80)
  return val

def patch(m, n, v):
  m = list(m)
  m[1] = n
  m[2] = v
  return m

def test():
  print(run([1,9,10,3,2,3,11,0,99,30,40,50]))
  print(run([1,0,0,0,99]))
  print(run([2,3,0,3,99]))
  print(run([2,4,4,5,99,0]))
  print(run([1,1,1,4,99,5,6,0,99]))
  print(run([3,0,4,0,99]))
  print(run([1002,4,3,4,33]))
  print(run([1101,100,-1,4,0]))
  print('input 8')
  print(run([3,9,8,9,10,9,4,9,99,-1,8]))
  print('input <8')
  print(run([3,9,7,9,10,9,4,9,99,-1,8]))
  print('input 8')
  print(run([3,3,1108,-1,8,3,4,3,99]))
  print('input <8')
  print(run([3,3,1107,-1,8,3,4,3,99]))
  print('cast to bool')
  print(run([3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]))
  print('cast to bool')
  print(run([3,3,1105,-1,9,1101,0,0,12,4,12,99,1]))

def main():
  f = file('program.txt')
  s = f.read().strip()
  ss = s.split(',')
  xs = [int(x) for x in ss]
  print(run(xs))

main()

# 1: 4330636
# 2: 10428568
