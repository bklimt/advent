
input = file('input.txt').read().strip().split()

target = int(input[0])
busses = [int(x) for x in input[1].split(',') if x != 'x']
modded = [(x - (target % x), x) for x in busses]
modded.sort()

print target
print busses
print modded
print modded[0][0] * modded[0][1]
