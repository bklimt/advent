
input = [0,12,6,13,20,1,17]

seen = {
}

i = 0

curr = 0
for x in input:
  prev = curr
  curr = x
  if i > 0:
    seen[prev] = i - 1
  i = i + 1
  # print `i` + ' -> ' + `curr` + ': ' + `seen`

while i < 2020:
  prev = curr
  if curr not in seen:
    curr = 0
  else:
    curr = (i - seen[curr]) - 1
  seen[prev] = i - 1
  i = i + 1
  # print `i` + ' -> ' + `curr` + ': ' + `seen`

print curr
