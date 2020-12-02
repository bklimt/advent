lines = file('input.txt').read().strip().split('\n')

valid = 0
for line in lines:
  print line
  rule, pw = line.split(': ')
  bounds, letter = rule.split(' ')
  l, h = bounds.split('-')
  l = int(l)
  h = int(h)
  c = pw.count(letter)
  if c >= l and c <= h:
    valid = valid + 1

print valid