
# p1 = [9, 2, 6, 3, 1]
# p2 = [5, 8, 4, 7, 10]

p1 = [12, 48, 26, 22, 44, 16, 31, 19, 30, 10, 40, 47, 21, 27, 2, 46, 9, 15, 23, 6, 50, 28, 5, 42, 34]
p2 = [14, 45, 4, 24, 1, 7, 36, 29, 38, 33, 3, 13, 11, 17, 39, 43, 8, 41, 32, 37, 35, 49, 20, 18, 25]

while len(p1) != 0 and len(p2) != 0:
  c1 = p1[0]
  c2 = p2[0]
  p1 = p1[1:]
  p2 = p2[1:]
  if c1 > c2:
    p1.append(c1)
    p1.append(c2)
  else:
    p2.append(c2)
    p2.append(c1)

print p1
print p2

ans = 0
ans = ans + sum([(len(p2)-i) * p2[i] for i in range(len(p2))])
ans = ans + sum([(len(p1)-i) * p1[i] for i in range(len(p1))])
print ans