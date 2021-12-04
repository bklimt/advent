
import math

def mcb(lines):
  #print('lines =', lines)
  counts = []
  for line in lines:
    for i, bit in enumerate(line):
      while i >= len(counts):
        counts.append(0)
      if bit == '1':
        counts[i] = counts[i] + 1
  target = math.ceil(len(lines)/2)
  #print('counts =', counts)
  #print('target =', target)
  return ''.join(['1' if x >= target else '0' for x in counts])

def inverse(n):
  return ''.join(['0' if x == '1' else '1' for x in n])

def part2(lines, inv, i=0):
  #print('lines =', lines)
  if len(lines) == 1:
    return lines[0]
  if len(lines) == 0:
    return None
  pattern = mcb(lines)
  #print('pattern =', pattern)
  if inv:
    pattern = inverse(pattern)
  #print('pattern =', pattern, 'inv =', inv, 'i =', i)
  matching = [line for line in lines if line[i] == pattern[i]]
  return part2(matching, inv, i+1)

def main():
  f = open('input.txt')
  lines = [line.strip() for line in f if line.strip() != '']
  gamma = mcb(lines)
  epsilon = inverse(gamma)
  gamma = int(gamma, 2)
  epsilon = int(epsilon, 2)
  print('gamma =', gamma)
  print('epsilon =', epsilon)
  print('ans1 =', gamma * epsilon)
  oxygen = part2(lines, False)
  co2 = part2(lines, True)
  oxygen = int(oxygen, 2)
  co2 = int(co2, 2)
  print('oxygen =', oxygen)
  print('co2 =', co2)
  print('ans2 =', oxygen * co2)

main()

#print(mcb(['00100', '01111', '00111', '00010', '01010']))
