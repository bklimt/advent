
aim = 0
x = 0
y = 0

f = open('input.txt')
for line in f:
  direction, amount = line.strip().split(' ')
  amount = int(amount)
  if direction == 'forward':
    x = x + amount
    y = y + (amount * aim)
  if direction == 'down':
    aim = aim + amount
  if direction == 'up':
    aim = aim - amount

print(x * y)

