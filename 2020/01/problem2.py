nums = [int(x) for x in file('input.txt').read().strip().split('\n')]
for i in range(len(nums)):
  for j in range(i+1, len(nums)):
    for k in range(j+1, len(nums)):
      if nums[i] + nums[j] + nums[k] == 2020:
        print nums[i]*nums[j]*nums[k]