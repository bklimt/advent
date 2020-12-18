

def process(s, indent=''):
  s = ''.join(s.split())
  # print indent + s
  # Start by removing parens.
  while True:
    start = s.rfind('(')
    if start == -1:
      break
    start = start + 1
    end = start + s[start:].find(')')
    n = process(s[start:end], indent + '  ')
    s = s[:start-1] + str(n) + s[end+1:]
    # print indent + s
  # There are no more parens.
  factors = s.split('*')
  product = 1
  for factor in factors:
    terms = factor.split('+')
    sum = 0
    for term in terms:
      sum = sum + int(term)
    product = product * sum
  # print indent + '<- ' + str(product)
  return product

def main():
  lines = file('input.txt').read().strip().split('\n')
  total = 0
  for line in lines:
    n = process(line)
    print line + ' = ' + str(n)
    total = total + n
  print total  

main()
