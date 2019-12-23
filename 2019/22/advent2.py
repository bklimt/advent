
size = 10007
ans = 2019

# size = 119315717514047
# ans = 2020

def reverse():
  global ans
  ans = (size-1)-ans

def cut(n):
  global ans
  if n < 0:
    n = n + size
  ans = ans - n
  ans = ans + size
  ans = ans % size

def inc(n):
  global ans
  ans = ans * n
  ans = ans % size

def process():
  with open('input.txt') as f:
    for line in f:
      line = line.strip()
      print(line)
      if line == 'deal into new stack':
        reverse()
      elif line[:4] == 'cut ':
        cut(int(line[4:]))
      elif line[:20] == 'deal with increment ':
        inc(int(line[20:]))
      else:
        raise 'wut'

process()
print(ans)
