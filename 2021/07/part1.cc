
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>

#include "2021/base/util.h"
#include "absl/status/status.h"

template <typename T>
void PrintNumbers(const std::vector<T>& numbers) {
  for (auto n : numbers) {
    std::cout << absl::StrCat(n, " ");
  }
  std::cout << std::endl;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/07/input.txt"));
  ASSIGN_OR_RETURN(auto numbers, ReadNumbers(in));

  std::sort(numbers.begin(), numbers.end());

  decltype(numbers) left_sums(numbers.size());
  decltype(numbers) right_sums(numbers.size());

  left_sums[0] = 0;
  right_sums[right_sums.size() - 1] = 0;
  for (size_t i = 1; i < numbers.size(); i++) {
    size_t j = (right_sums.size() - 1) - i;
    left_sums[i] = left_sums[i - 1] + (i * (numbers[i] - numbers[i - 1]));
    right_sums[j] = right_sums[j + 1] + (i * (numbers[j + 1] - numbers[j]));
  }

  decltype(numbers) sums(numbers.size());
  for (size_t i = 0; i < numbers.size(); i++) {
    sums[i] = left_sums[i] + right_sums[i];
  }

  // PrintNumbers(numbers);
  // PrintNumbers(left_sums);
  // PrintNumbers(right_sums);
  // PrintNumbers(sums);

  auto ans = std::min_element(sums.begin(), sums.end());
  if (ans == sums.end()) {
    return absl::InternalError("list must be empty");
  }
  std::cout << "Answer: " << *ans << std::endl;

  return absl::OkStatus();
}
