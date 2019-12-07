
def has_twins(d):
  s = str(d)
  for i in range(len(s)-1):
    if s[i] == s[i+1]:
      if i+2 < len(s):
        if s[i+1] == s[i+2]:
          continue
      if i > 0:
        if s[i-1] == s[i]:
          continue
      return True
  return False

def is_mono(d):
  s = str(d)
  for i in range(len(s)-1):
    if s[i+1] < s[i]:
      return False
  return True

t = 0

for i in range(136760, 595731):
  if not has_twins(i):
    continue
  if not is_mono(i):
    continue
  t = t + 1

print(t)

# 1: 1873
# 2: 1264

print(has_twins(112233))
print(has_twins(123444))
print(has_twins(111122))
