
from collections import Counter

adapters = [int(x) for x in file('input.txt').read().strip().split()]

def part1():
  xs = sorted(adapters)
  xs.append(xs[-1] + 3)
  ds = [xs[i]-xs[i-1] for i in range(1, len(xs))]
  ds = [xs[0]] + ds
  cs = Counter(ds)
  print cs[1] * cs[3]

def main():
  part1()

main()
