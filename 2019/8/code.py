
width = 25
height = 6
size = width * height

image = open('input.txt').read().strip()

min0 = size + 1
result = 0
for i in range(len(image)/size):
  start = i*size
  end = start+size
  layer = image[start:end]
  zeroes = len([x for x in layer if x == '0'])
  print("0[%d:%d] = %d" % (start, end, zeroes))
  if zeroes < min0:
    min0 = zeroes
    ones = len([x for x in layer if x == '1'])
    twos = len([x for x in layer if x == '2'])
    result = ones * twos

print("zeroes: %d\nresult: %d" % (zeroes, result))

# 1690
