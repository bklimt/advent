
import string

def mapchar(c):
  if c == 'F' or c == 'L':
    return '0'
  if c == 'B' or c == 'R':
    return '1'
  raise Exception('oh no!')

def parse1(line):
  s = ''.join([mapchar(c) for c in line])
  return string.atoi(s[:7], 2), string.atoi(s[-3:], 2)

def parse2(line):
  s = ''.join([mapchar(c) for c in line])
  return string.atoi(s, 2)

def main():
  input = file('input.txt').read().strip().split()
  min = string.atoi('1111111111', 2)
  max = 0
  found = [False] * 0b10000000000
  for line in input:
    sid = parse2(line)
    if sid > max:
      max = sid
    if sid < min:
      min = sid
    found[sid] = True
  print 'min =', min
  print 'max =', max
  for i in range(min + 1, max):
    if not found[i]:
      print 'missing =', i

main()
