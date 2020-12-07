
groups = file('input.txt').read().strip().split('\n\n')

sum = 0

for group in groups:
  found = {}
  for c in group:
    if c != '\n':
      found[c] = True
  sum = sum + len(found)

print sum
