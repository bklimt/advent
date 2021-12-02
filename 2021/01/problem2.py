
f = open('input.txt')
previous1 = None
previous2 = None
previous3 = None
count = 0
for line in f:
  depth = int(line)
  if (previous1 is not None) and (previous2 is not None) and (previous3 is not None) and previous1 < depth:
    count = count + 1
  previous1, previous2, previous3 = previous2, previous3, depth
print(count)
