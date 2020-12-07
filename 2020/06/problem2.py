
groups = file('input.txt').read().strip().split('\n\n')

sum = 0

for group in groups:
  people = group.split()
  found = {}
  for c in people[0]:
    found[c] = True
  for person in people[1:]:
    for c in found.keys():
      if c not in person:
        del found[c]
  sum = sum + len(found)

print sum
