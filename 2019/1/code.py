
def fuel(m):
  f = m/3-2
  if f <= 0:
    return 0
  return f + fuel(f)

def main():
  f = open('input.txt')
  s = f.read().strip()
  xs = [int(x) for x in s.split()]
  ys = [fuel(x) for x in xs]
  print(repr(xs))
  print(repr(ys))
  t = 0
  for y in ys:
    t = t + y
  print(t)

main()

print(fuel(14))
print(fuel(1969))
print(fuel(100756))

# 1: 3223398
# 2: 4832253
