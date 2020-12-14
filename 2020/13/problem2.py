
import numpy

input = file('input.txt').read().strip().split()

busses = input[1].split(',')
busses = [[i, int(busses[i])] for i in range(len(busses)) if busses[i] != 'x']
print 'busses = ' + repr(busses)

busses[0][0] = busses[0][1]
print 'busses = ' + repr(busses)

def is_solution(target):
  # x % -b == -i
  # return all([-b[0] == target % -b[1] for b in busses])
  # x % b == b - i
  return all([b[1] - b[0] == target % b[1] for b in busses])

def main():
  largest = max(busses, key = lambda b: b[1])
  print 'largest = ' + repr(largest)
  i = largest[1] - largest[0]
  while True:
    if is_solution(i):
      print 'ans = ' + repr(i)
      return
    i = i + largest[1]

# print is_solution(1068780)
print is_solution(1068781)
# print is_solution(1068782)

# main()

# [[7, 7], [1, 13], [4, 59], [6, 31], [7, 19]]

# 7 * x -t = 7
# 13 * y - t = 1
# 59 * z - t = 5
# 31 * v - t = 6
# 19 * w - t = 7


print ans
