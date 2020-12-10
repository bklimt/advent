
from collections import Counter
from functools import reduce
from math import factorial

adapters = [int(x) for x in file('input.txt').read().strip().split()]

# 458 -> 458
# 4569 -> 4569 469 2->2
# 4567A -> 4567A 467A 47A 457A 3->4
# 45678B -> 45678B 4678B 468B 478B 4578B 4568B 458B 4->7 

span_to_factor = {
  '11': 2,
  '111': 4,
  '1111': 7,
}

def main():
  # part 1
  xs = sorted(adapters)
  xs.append(xs[-1] + 3)
  print xs
  ds = [xs[i]-xs[i-1] for i in range(1, len(xs))]
  ds = [xs[0]] + ds
  print ds
  cs = Counter(ds)
  print cs
  print cs[1] * cs[3]

  # part 2
  dss = ''.join(str(x) for x in ds)
  print dss
  spans = [x for x in dss.split('3') if len(x) > 1]
  print spans
  factors = [span_to_factor[x] for x in spans]
  print factors
  ans = reduce(lambda x, y: x * y, factors)
  print ans

main()
