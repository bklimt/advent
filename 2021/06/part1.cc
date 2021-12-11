
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_split.h"

absl::StatusOr<std::vector<int>> ReadNumbers(std::ifstream& in) {
  auto line1 = ReadLine(in);
  if (!line1) {
    return absl::InvalidArgumentError("expected list of numbers");
  }

  std::vector<absl::string_view> parts = absl::StrSplit(*line1, ",");
  std::vector<int> numbers;
  for (auto& part : parts) {
    int n;
    if (!absl::SimpleAtoi(part, &n)) {
      return absl::InternalError(absl::StrCat("invalid number: ", part));
    }
    numbers.push_back(n);
  }
  return numbers;
}

void PrintCounts(const std::vector<int64_t>& counts) {
  for (int i = 0; i < static_cast<int>(counts.size()); i++) {
    std::cout << absl::StrCat(i, ":", counts[i], " ");
  }
  std::cout << std::endl;
}

int64_t Sum(const std::vector<int64_t>& counts) {
  int64_t s = 0;
  for (auto n : counts) {
    s += n;
  }
  return s;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/06/input.txt"));
  ASSIGN_OR_RETURN(auto numbers, ReadNumbers(in));

  std::vector<int64_t> counts(9, 0);

  // Load in the input.
  for (auto n : numbers) {
    if (n < 0 || n > 8) {
      return absl::InvalidArgumentError(absl::StrCat("invalid number: ", n));
    }
    counts[n]++;
  }
  std::cout << "day " << 0 << ": ";
  PrintCounts(counts);

  // Iterate N times.
  for (int j = 0; j < 256; j++) {
    int64_t finished = counts[0];
    for (size_t i = 1; i < counts.size(); i++) {
      counts[i - 1] = counts[i];
    }
    counts[6] += finished;
    counts[counts.size() - 1] = finished;
    std::cout << "day " << j + 1 << ": ";
    PrintCounts(counts);
  }
  std::cout << "Total: " << Sum(counts) << std::endl;

  return absl::OkStatus();
}
