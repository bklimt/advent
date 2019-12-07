import sys

running = False
memory = [99]

input = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,13,23,1,23,10,27,1,13,27,31,2,31,10,35,1,35,9,39,1,39,13,43,1,13,43,47,1,47,13,51,1,13,51,55,1,5,55,59,2,10,59,63,1,9,63,67,1,6,67,71,2,71,13,75,2,75,13,79,1,79,9,83,2,83,10,87,1,9,87,91,1,6,91,95,1,95,10,99,1,99,13,103,1,13,103,107,2,13,107,111,1,111,9,115,2,115,10,119,1,119,5,123,1,123,2,127,1,127,5,0,99,2,14,0,0]

def quit():
  global running
  print("exiting")
  running = False

def add(a1, a2, a3):
  d1 = memory[a1]
  d2 = memory[a2]
  d3 = d1 + d2
  memory[a3] = d3
  print("[%d] %d + [%d] %d = [%d] %d" % (a1, d1, a2, d2, a3, d3))

def multiply(a1, a2, a3):
  d1 = memory[a1]
  d2 = memory[a2]
  d3 = d1 * d2
  memory[a3] = d3
  print("[%d] %d * [%d] %d = [%d] %d" % (a1, d1, a2, d2, a3, d3))

def process(i):
  opcode = memory[i]
  if opcode == 99:
    quit()
    return i+1
  if opcode == 1:
    add(memory[i+1], memory[i+2], memory[i+3])
    return i+4
  if opcode == 2:
    multiply(memory[i+1], memory[i+2], memory[i+3])
    return i+4
  print("unknown opcode: %d" % opcode)
  quit()

def run(m):
  global memory
  global running
  memory = m
  running = True
  i = 0
  while running:
    i = process(i)
    print(memory)
  return memory[0]

def patch(m, n, v):
  m = list(m)
  m[1] = n
  m[2] = v
  return m

def main():
  #run([1,9,10,3,2,3,11,0,99,30,40,50])
  #run([1,0,0,0,99])
  #run([2,3,0,3,99])
  #run([2,4,4,5,99,0])
  #run([1,1,1,4,99,5,6,0,99])
  for n in range(100):
    for v in range(100):
      result = run(patch(input, n, v))
      print("result: %d, noun: %d, verb: %d" % (result, n, v))
      if result == 19690720:
        sys.exit(0)

main()

# 1: 4330636
# 2: 6086
