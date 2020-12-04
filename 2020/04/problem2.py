
required = [x.strip()[:3] for x in """
    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)
""".strip().split('\n') if x.strip()[:3] != 'cid']
print required
print '\n'

def valid_hgt(x):
  try:
    h = int(x[:-2])
  except:
    return 'not an int: ' + repr(x[:-2])
  if x[-2:] == 'cm':
    if h < 150 or h > 193:
      return 'cm out of range: ' + repr(h)
    else:
      return None
  if x[-2:] == 'in':
    if h < 59 or h > 76:
      return 'in out of range: ' + repr(h)
    else:
      return None
  return 'not in or cm: ' + repr(x)

def valid_hcl(x):
  if x[0] != '#':
    return 'does not start with #'
  for c in x[1:]:
    if c not in '0123456789abcdef':
      return 'character ' + repr(c) + ' not in range'
  return None

def valid_pid(x):
  if len(x) != 9:
    return 'wrong length: ' + repr(x)
  for c in x:
    if c not in '0123456789':
      return 'character ' + repr(c) + ' not in range'
  return None

def valid_range(low, high):
  def f(x):
    if int(x) < low or int(x) > high:
      return 'number ' + repr(x) + ' out of range'
    return None
  return f

def valid_ecl(x):
  if x not in "amb blu brn gry grn hzl oth".split():
    return repr(x) + ' not in set'
  return None

validation = {
  'byr': valid_range(1920, 2002),
  'iyr': valid_range(2010, 2020),
  'eyr': valid_range(2020, 2030),
  'hgt': valid_hgt,
  'hcl': valid_hcl,
  'ecl': valid_ecl,
  'pid': valid_pid,
}

valid = 0
invalid = 0
s = file('input.txt').read().strip()
passports = [x.strip() for x in s.split('\n\n')]
for passport in passports:
  fields = [x.split(':') for x in passport.split()]
  keys = [field[0] for field in fields]
  data = dict(fields)
  v = True
  for key in required:
    if key not in keys:
      v = False
      print 'missing required key ' + repr(key)
      break
    else:
      err = validation[key](data[key])
      if err is not None:
        v = False
        print 'failed validation for ', key, ' in ', data
        print 'error: ' + err
        break
  if v:
    print 'valid: ' + repr(data)
    valid = valid + 1
  else:
    print 'invalid: ' + repr(data)
    invalid = invalid + 1

print valid
print invalid
