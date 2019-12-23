
size = 10007
cards = [i for i in range(size)]
buffer = [0]*size

def reverse():
  global cards
  cards = cards[::-1]

def cut(n):
  if n < 0:
    n = n + size
  for i in range(n):
    buffer[(size-n)+i] = cards[i]
  for i in range(n, size):
    buffer[i-n] = cards[i]
  for i in range(size):
    cards[i] = buffer[i]

def inc(n):
  dst = 0
  for src in range(size):
    buffer[dst] = cards[src]
    dst = dst + n
    dst = dst % size
  for i in range(size):
    cards[i] = buffer[i]

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

def search():
  for i in range(size):
    if cards[i] == 2019:
      print(i)

process()
search()
