#!python3

# Makes a graph of the input to day 20.

f = open("data/20/input.txt")
text = f.read()
lines = text.split('\n')
lines = [line.strip() for line in lines]
print('digraph problem20 {')
for line in lines:
    line = line.strip()
    if len(line) == 0:
        continue
    (lhs, rhs) = line.split(' -> ')
    name = lhs
    if lhs[0] == '&' or lhs[0] == '%':
        lhs = lhs[1:]
    print('  ' + lhs + ' [label="' + name + '"]')
    rhss = rhs.split(', ')
    for rhs in rhss:
        print('  ' + lhs + ' -> ' + rhs + ';')
print('}')
