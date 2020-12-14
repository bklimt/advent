
input = file('sample.txt').read().strip().split()

busses = input[1].split(',')
busses = [[i, int(busses[i])] for i in range(len(busses)) if busses[i] != 'x']
print 'busses = ' + repr(busses)

#busses[0][0] = busses[0][1]
busses = busses[0:2]
print 'busses = ' + repr(busses)

def is_solution(target):
  # x % -b == -i
  # return all([-b[0] == target % -b[1] for b in busses])
  # x % b == b - i
  return all([b[1] - b[0] == target % b[1] for b in busses])

def find_pairwise(p1, p2):
  i = 1
  while True:
    for j in range(i+1):
      print '%s -> %s' % ([j, i], [p1[1]*i+p1[0], p2[1]*j+p2[0]])
      if p1[1]*i+p1[0] == p2[1]*j+p2[0]:
        return [j, i]
    i = i + 1
    if i > 25:
      return False

def find_t(factor1, offset1, factor2):
  y1 = factor1 - offset1
  y2 = factor1 * 2 - offset1
  mod1 = y1 % factor2
  mod2 = y2 % factor2
  diff = mod2 - mod1
  print y1, y2, mod1, mod2, diff

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

# print find_pairwise([0, 7], [1, 13])

# main()

find_t(13, 1, 7)
