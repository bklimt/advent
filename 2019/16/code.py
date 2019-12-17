
def pattern(digit, n):
  x = [0]*digit + [1]*digit + [0]*digit + [-1]*digit
  while len(x) < n+1:
    x = x + x
  return x[1:n+1]

# O(n)
def update(input, pattern, n):
  x = 0
  for i in range(n):
    x = x + input[i] * pattern[i]
  x = abs(x)%10
  return x

# O(n^2)
def phase(n, input, patterns, output):
  print("n = %d" % n)
  for i in range(n):
    # print("computing digit %d" % (i+1))
    # print("pattern = %s" % repr(patterns[i]))
    output[i] = update(input, patterns[i], n)

def main():
  # start = [1,2,3,4,5,6,7,8]
  # start = [int(x) for x in "80871224585914546619083218645595"]
  start = [int(x) for x in open("input.txt").read()*10000]
  n = len(start)
  patterns = [None]*n
  for i in range(n):
    patterns[i] = pattern(i+1, n)

  signal = start
  output = [0]*n
  for i in range(100):
    phase(n, signal, patterns, output)
    signal = output

  print(''.join([str(x) for x in signal]))

main()

# 1: 58100105
