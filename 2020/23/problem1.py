
import bisect

def take3(input, pos):
  if pos < len(input)-3:
    return input[:pos] + input[pos+3:], input[pos:pos+3]
  else:
    first = 3-(len(input)-pos)
    return input[first:pos], input[pos:] + input[:first]

def find_destination(input, current):
  # Find the next lowest number.
  print 'remaining: ' + ' '.join([str(x) for x in input])
  sorted_input = sorted(input)
  print 'sorted remaining: ' + ' '.join([str(x) for x in sorted_input])
  print 'looking for %d' % (current-1)
  next_lowest_pos = bisect.bisect(sorted_input, current-1)
  print 'found bisect at %d' % next_lowest_pos
  if next_lowest_pos == 0:
    next_lowest_pos = len(sorted_input) - 1
  else:
    next_lowest_pos = next_lowest_pos - 1
  next_lowest = sorted_input[next_lowest_pos]
  print 'destination: %d' % next_lowest

  # Now find that position in the input.
  return input.index(next_lowest)

def place(input, pos, current, picked_up, destination):
  result = input[:destination+1] + picked_up + input[destination+1:]
  print 'placed: ' + ' '.join([str(x) for x in result])
  while result[pos] != current:
    result = result[1:] + result[:1]
    print 'rotated: ' + ' '.join([str(x) for x in result])
  return result


def turn(input, pos):
  current = input[pos]
  input, picked_up = take3(input, pos+1)
  print 'pick up: ' + ', '.join([str(x) for x in picked_up])
  destination = find_destination(input, current)
  print 'destination index: %d' % destination
  input = place(input, pos, current, picked_up, destination)
  pos = (pos + 1) % len(input)
  return input, pos

def print_cups(input, pos):
  print 'cups:',
  for j in range(len(input)):
    if j == pos:
      print '(%d)' % input[j], 
    else:
      print input[j],
  print

def main():
  # input = "389125467"
  input = "871369452"
  input = [int(x) for x in list(input)]
  pos = 0
  for i in range(100):
    print '-- move %d --' % i
    print_cups(input, pos)
    input, pos = turn(input, pos)
    print
  print '-- final --'
  print_cups(input, pos)

  start = input.index(1)
  input = input[start:] + input[:start]
  print ''.join([str(x) for x in input[1:]])

main()
