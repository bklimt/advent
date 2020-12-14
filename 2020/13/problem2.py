
input = file('input.txt').read().strip().split()

busses = input[1].split(',')
busses = [[i, int(busses[i])] for i in range(len(busses)) if busses[i] != 'x']
busses = [[bus[0] % bus[1], bus[1]] for bus in busses]
print 'busses = ' + repr(busses)

def main():
  t = busses[0][0]
  dt = busses[0][1]
  for bus in busses[1:]:
    while -bus[0] != t % -bus[1]:
      t += dt
    dt = dt * bus[1]
    print 'caught bus at t=' + repr(t) + ', dt=' + repr(dt)
  print t

main()
