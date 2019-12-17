
def pattern(digit, n):
  x = [0]*digit + [1]*digit + [0]*digit + [-1]*digit
  while len(x) < n+1:
    x = x + x
  return x[1:n+1]

def update(input, pattern):
  n = 0
  for i in range(len(input)):
    n = n + input[i] * pattern[i % len(pattern)]
  n = abs(n)%10
  return n

def phase(input):
  n = len(input)
  output = [0]*n
  print("n = %d" % n)
  for i in range(n):
    # print("computing digit %d" % (i+1))
    p = pattern(i+1, n)
    # print("pattern = %s" % repr(p))
    output[i] = update(input, p)
  return output

# start = [1,2,3,4,5,6,7,8]
# start = [int(x) for x in "80871224585914546619083218645595"]
start = [int(x) for x in open("input.txt").read()]

signal = start
for i in range(100):
  signal = phase(signal)
  print(''.join([str(x) for x in signal]))

# 1: 58100105
