
counts = []
lines = 0

f = open('input.txt')
for line in f:
  line = line.strip()
  lines = lines + 1
  for i, bit in enumerate(line):
    while i >= len(counts):
      counts.append(0)
    if bit == '1':
      counts[i] = counts[i] + 1
  print(line)
  print(counts)

  target = round(lines/2)

  gamma = ''.join(['1' if x >= target else '0' for x in counts])
  epsilon = ''.join(['1' if x < target else '0' for x in counts])

  gamma = int(gamma, 2)
  epsilon = int(epsilon, 2)

  print(gamma)
  print(epsilon)
  print(gamma * epsilon)
