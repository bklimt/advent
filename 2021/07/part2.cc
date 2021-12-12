
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

int ComputeCost(int pos1, int pos2) {
  if (pos1 > pos2) {
    return ComputeCost(pos2, pos1);
  }
  int dist = pos2 - pos1;
  return (dist * (dist + 1)) / 2;
}

int ComputeTotalCost(int position, const std::vector<int>& numbers) {
  int total = 0;
  for (auto n : numbers) {
    total += ComputeCost(position, n);
  }
  return total;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/07/input.txt"));
  ASSIGN_OR_RETURN(auto numbers, ReadNumbers(in));

  std::sort(numbers.begin(), numbers.end());
  // PrintNumbers(numbers);
  // std::cout << std::endl;

  std::vector<std::vector<int>> distance(numbers.size(),
                                         std::vector<int>(numbers.size(), 0));

  for (size_t i = 0; i < numbers.size(); i++) {
    for (size_t j = 0; j < i; j++) {
      int dist = numbers[i] - numbers[j];
      int cost = (dist * (dist + 1)) / 2;
      distance[i][j] = cost;
      distance[j][i] = cost;
    }
  }

  /*
  for (size_t i = 0; i < numbers.size(); i++) {
    PrintNumbers(distance[i]);
  }
  std::cout << std::endl;
  */

  std::vector<int> sums;
  for (size_t i = 0; i < numbers.size(); i++) {
    int sum = 0;
    for (size_t j = 0; j < numbers.size(); j++) {
      sum += distance[i][j];
    }
    sums.push_back(sum);
  }
  // PrintNumbers(sums);
  // std::cout << std::endl;

  auto ans = std::min_element(sums.begin(), sums.end());
  if (ans == sums.end()) {
    return absl::InternalError("list must be empty");
  }
  std::cout << "Answer 1: " << *ans << std::endl;

  size_t pos = std::distance(sums.begin(), ans);
  if (pos == 0) {
    return absl::InternalError("first element");
  }
  if (pos == sums.size() - 1) {
    return absl::InternalError("last element");
  }

  size_t min = numbers[pos - 1];
  size_t max = numbers[pos + 1];
  std::cout << "Looking between " << min << " and " << max << std::endl;

  for (int i = min; i <= max; i++) {
    std::cout << i << ": " << ComputeTotalCost(i, numbers) << std::endl;
  }

  return absl::OkStatus();
}
