
def has_twins(d):
  s = str(d)
  for i in range(len(s)):
    for j in range(len(s)):
      if i != j:
        if s[i] == s[j]:
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
