
code = file('input.txt').read().strip().split('\n')

def run(code):
  visited = [False] * len(code)
  ip = 0
  ax = 0
  while True:
    if ip == len(code):
      return ax, True

    if visited[ip]:
      return ax, False
    visited[ip] = True

    line = code[ip]
    op, arg = line.split()
    arg = int(arg)
    if op == 'acc':
      ax = ax + arg
      ip = ip + 1
    elif op == 'nop':
      ip = ip + 1
    elif op == 'jmp':
      ip = ip + arg
    else:
      raise Exception('oh no! ' + repr(line))

#ax, success = run(code)
#print ax, success

def main():
  for i in range(len(code)):
    op, arg = code[i].split()
    new_inst = None
    if op == 'nop':
      new_inst = 'jmp ' + arg
    if op == 'jmp':
      new_inst = 'nop ' + arg
    if new_inst is None:
      continue

    new_code = code[:i] + [new_inst] + code[i+1:]
    ax, success = run(new_code)
    if success:
      print ax
      return

main()
