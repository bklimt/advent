
def score(hand):
  return sum([(len(hand)-i) * hand[i] for i in range(len(hand))])

game_cache = {}

def game(p1, p2, depth=0):
  cache_key = (tuple(p1), tuple(p2))
  if cache_key in game_cache:
    return game_cache[cache_key]

  indent = '  ' * depth
  # print '%sGame %d' % (indent, depth)
  # print 'p1 = ' + repr(p1)
  # print 'p2 = ' + repr(p2)
  seen = {}

  while len(p1) != 0 and len(p2) != 0:
    # print 'Round'
    # print 'p1 = ' + repr(p1)
    # print 'p2 = ' + repr(p2)

    t1 = tuple(p1)
    t2 = tuple(p2)
    if (t1, t2) in seen:
      # print indent + 'Player 1 wins by base case!'
      game_cache[cache_key] = 1
      return 1
    seen[(t1, t2)] = True

    c1 = p1[0]
    c2 = p2[0]
    p1 = p1[1:]
    p2 = p2[1:]

    if len(p1) >= c1 and len(p2) >= c2:
      winner = game(list(p1[:c1]), list(p2[:c2]), depth + 1)
      # print 'Player %d won a round.' % winner
    else:
      # print '%d vs %d' % (c1, c2)
      if c1 > c2:
        winner = 1
      else:
        winner = 2

    if winner == 1:
      # print 'Player 1 won a round.'
      p1.append(c1)
      p1.append(c2)
    else:
      # print 'Player 2 won a round.'
      p2.append(c2)
      p2.append(c1)

  # print p1
  # print p2

  ans = 0
  ans = ans + sum([(len(p2)-i) * p2[i] for i in range(len(p2))])
  ans = ans + sum([(len(p1)-i) * p1[i] for i in range(len(p1))])
  print '%s%d' % (indent, ans)

  if len(p1) == 0:
    # print indent + 'Player 2 won the game.'
    game_cache[cache_key] = 2
    return 2
  else:
    # print indent + 'Player 1 won the game.'
    game_cache[cache_key] = 1
    return 1

def main():
  input = 4
  if input == 1:
    p1 = [9, 2, 6, 3, 1]
    p2 = [5, 8, 4, 7, 10]
  elif input == 2:
    p1 = [43, 19]
    p2 = [2, 29, 14]
  elif input == 3:
    p1 = [9, 2, 6, 3, 1]
    p2 = [5, 8, 4, 7, 10]
  else:
    p1 = [12, 48, 26, 22, 44, 16, 31, 19, 30, 10, 40, 47, 21, 27, 2, 46, 9, 15, 23, 6, 50, 28, 5, 42, 34]
    p2 = [14, 45, 4, 24, 1, 7, 36, 29, 38, 33, 3, 13, 11, 17, 39, 43, 8, 41, 32, 37, 35, 49, 20, 18, 25]

  game(p1, p2)

main()
