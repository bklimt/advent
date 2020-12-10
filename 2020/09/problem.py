
data = file('input.txt').read().strip().split()
data = [int(x) for x in data]

window = 25
#window = 5

def problem1():
  for i in range(window, len(data)):
    found = False
    for j in range(i - window, i):
      for k in range(i - window, i):
        if j != k:
          if data[j] + data[k] == data[i]:
            found = True
    if not found:
      print 'problem 1:', data[i]
      return data[i]

class SpanData:
  def __init__(self, start, end, sum):
    self.start = start
    self.end = end
    self.sum = sum
  
  def extend(self, i, num):
    if i != self.end:
      raise Exception('oh no! ' + repr(i) + ' != ' + repr(self.end))
    self.end = i + 1
    self.sum = self.sum + num

def problem2(target):
  sums = []

  for i in range(len(data)):
    # Add the latest to the window.
    for j in range(len(sums)):
      sums[j].extend(i, data[i])
    sums.append(SpanData(i, i + 1, data[i]))

    # Check for the right answer.
    for sum in sums:
      if sum.end - sum.start > 1 and sum.sum == target:
        print 'answer range: ' + repr([sum.start, sum.end])
        values = data[sum.start:sum.end]
        print 'answer values: ' + repr(values)
        print 'result: ' + repr(min(values) + max(values))
        return

    # Trim out windows that are too big.
    sums = [sum for sum in sums if sum.sum <= target]

def main():
  target = problem1()
  problem2(target)

main()
