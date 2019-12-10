
width = 25
height = 6
size = width * height

bytes = open('input.txt').read().strip()

image = [['2' for x in range(width)] for y in range(height)]

def pix(p):
  if p == '0':
    return ' '
  if p == '1':
    return '#'
  if p == '2':
    return '?'

def printimage(img):
  print('\n'.join(''.join([pix(p) for p in row]) for row in img))

for i in range(len(bytes)/size):
  start = i*size
  end = start+size
  layer = bytes[start:end]
  for x in range(width):
    for y in range(height):
      if image[y][x] == '2':
        image[y][x] = layer[y*width + x]
  printimage(image)
  print('')

# ZPZUB
