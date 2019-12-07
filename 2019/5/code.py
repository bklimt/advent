import sys

program = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,13,23,1,23,10,27,1,13,27,31,2,31,10,35,1,35,9,39,1,39,13,43,1,13,43,47,1,47,13,51,1,13,51,55,1,5,55,59,2,10,59,63,1,9,63,67,1,6,67,71,2,71,13,75,2,75,13,79,1,79,9,83,2,83,10,87,1,9,87,91,1,6,91,95,1,95,10,99,1,99,13,103,1,13,103,107,2,13,107,111,1,111,9,115,2,115,10,119,1,119,5,123,1,123,2,127,1,127,5,0,99,2,14,0,0]

class Computer:
  def __init__(self, memory):
    self.running = True
    self.memory = memory
    self.ip = 0

  def log(self, s):
    print(s)

  def quit(self):
    self.log("exiting")
    self.running = False

  def add(self, a1, a2, a3):
    d1 = self.memory[a1]
    d2 = self.memory[a2]
    d3 = d1 + d2
    self.memory[a3] = d3
    self.log("[%d] %d + [%d] %d = [%d] %d" % (a1, d1, a2, d2, a3, d3))

  def multiply(self, a1, a2, a3):
    d1 = self.memory[a1]
    d2 = self.memory[a2]
    d3 = d1 * d2
    self.memory[a3] = d3
    self.log("[%d] %d * [%d] %d = [%d] %d" % (a1, d1, a2, d2, a3, d3))

  def input(self, a1):
    d1 = 0
    self.memory[a1] = d1
    self.log("[%a] = %d", (a1, d1))

  def fetch(self):
    d1 = self.memory[self.ip]
    self.ip = self.ip + 1
    return d1

  def process(self):
    opcode = self.fetch()
    if opcode == 99:
      self.quit()
    elif opcode == 1:
      a = self.fetch()
      b = self.fetch()
      c = self.fetch()
      self.add(a, b, c)
    elif opcode == 2:
      a = self.fetch()
      b = self.fetch()
      c = self.fetch()
      self.multiply(a, b, c)
    elif opcode == 3:
      a = self.fetch()
      self.input(a)
    else:
      self.log("unknown opcode: %d" % opcode)
      self.quit()

  def run(self):
    self.log("running %s" % self.memory)
    self.running = True
    self.ip = 0
    while self.running:
      self.process()
      self.log('state: %s' % self.memory)
    self.log("halted.")
    return self.memory[0]

def run(m):
  print('*' * 80)
  comp = Computer(m)
  val = comp.run()
  print('*' * 80)
  return val

def patch(m, n, v):
  m = list(m)
  m[1] = n
  m[2] = v
  return m

def main():
  print(run([1,9,10,3,2,3,11,0,99,30,40,50]))
  print(run([1,0,0,0,99]))
  print(run([2,3,0,3,99]))
  print(run([2,4,4,5,99,0]))
  print(run([1,1,1,4,99,5,6,0,99]))
  for n in range(100):
    for v in range(100):
      result = run(patch(program, n, v))
      print("result: %d, noun: %d, verb: %d" % (result, n, v))
      if result == 19690720:
        sys.exit(0)

main()

# 1: 4330636
# 2: 6086
