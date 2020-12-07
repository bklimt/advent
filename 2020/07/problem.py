
def strip_bag_suffix(s):
  if s[-4:] == ' bag':
    return s[:-4]
  if s[-5:] == ' bags':
    return s[:-5]
  raise Exception('oh no! ' + s)

def split_bag(s):
  count, bag = s.split(' ', 1)
  count = int(count)
  return BagCount(count, bag)

class BagCount:
  def __init__(self, count, bag):
    self.bag = bag
    self.count = count
  def __str__(self):
    return self.count + ' ' + self.bag
  def __repr__(self):
    return 'BagCount(' + repr(self.count) + ', ' + repr(self.bag) + ')'

class Rule:
  def __init__(self, lhs, rhs):
    self.lhs = lhs
    self.rhs = rhs
  def __str__(self):
    return self.lhs + ' -> ' + str(self.rhs)

def parse():
  rules = {}
  lines = file('input.txt').read().strip().split('\n')
  for line in lines:
    print line
    parts = line.split(' bags contain ')
    outer = parts[0]
    rest = parts[1][:-1]
    if rest == 'no other bags':
      rest = []
    else:
      rest = rest.split(', ')
      rest = [split_bag(strip_bag_suffix(x)) for x in rest]
    rule = Rule(outer, rest)
    print rule
    print ''
    rules[outer] = rule
  return rules

def problem1(rules):
  outer = {}
  outer['shiny gold'] = True
  changed = True
  while changed:
    changed = False
    for lhs in rules:
      rule = rules[lhs]
      if lhs not in outer:
        for rhs in rule.rhs:
          if rhs.bag in outer:
            outer[lhs] = True
            changed = True
  print len(outer) - 1

def count_closure(rules, bag):
  count = 1
  rule = rules[bag]
  for rhs in rule.rhs:
    count = count + rhs.count * count_closure(rules, rhs.bag)
  return count

def problem2(rules):
  count = count_closure(rules, 'shiny gold')
  print count

def main():
  rules = parse()
  problem2(rules)

main()
