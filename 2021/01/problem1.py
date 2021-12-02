
f = open('input.txt')
previous = None
count = 0
for line in f:
  depth = int(line)
  if (previous is not None) and previous < depth:
    count = count + 1
  previous = depth
print(count)
