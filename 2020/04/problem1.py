
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

valid = 0
invalid = 0
s = file('input.txt').read().strip()
passports = [x.strip() for x in s.split('\n\n')]
for passport in passports:
  fields = [x.split(':')[0] for x in passport.split()]
  v = True
  for key in required:
    if key not in fields:
      v = False
  if v:
    print 'valid'
    valid = valid + 1
  else:
    print 'invalid'
    invalid = invalid + 1

print valid
print invalid