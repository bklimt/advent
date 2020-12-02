lines = file('input.txt').read().strip().split('\n')

valid = 0
for line in lines:
  print line
  rule, pw = line.split(': ')
  bounds, letter = rule.split(' ')
  l, h = bounds.split('-')
  l = int(l)
  h = int(h)
  l = l - 1
  h = h - 1
  found = 0
  if l < len(pw) and pw[l] == letter:
    found = found + 1
  if h < len(pw) and pw[h] == letter:
    found = found + 1
  if found == 1:
    valid = valid + 1

print valid