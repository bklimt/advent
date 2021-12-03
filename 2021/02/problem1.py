
x = 0
y = 0

f = open('input.txt')
for line in f:
  direction, amount = line.strip().split(' ')
  amount = int(amount)
  if direction == 'forward':
    x = x + amount
  if direction == 'down':
    y = y + amount
  if direction == 'up':
    y = y - amount

print(x * y)

