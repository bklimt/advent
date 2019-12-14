
def parse_item(item):
  parts = item.split(' ')
  return (int(parts[0]), parts[1])

def parse_list(lst):
  return tuple(parse_item(item) for item in lst.split(', '))

def parse_eq(eq):
  parts = eq.split(' => ')
  return (parse_list(parts[0]), parse_item(parts[1]))

def parse_rules(s):
  s = s.strip()
  lines = s.split('\n')
  return tuple(parse_eq(line) for line in lines)

def parse_file(path):
  f = open(path)
  s = f.read()
  f.close()
  return parse_rules(s)

print(parse_file('input1.txt'))
