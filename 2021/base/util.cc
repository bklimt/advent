
#include "2021/base/util.h"

#include <fstream>

#include "absl/strings/str_split.h"

absl::StatusOr<std::ifstream> OpenFile(absl::string_view path) {
  std::ifstream in{path.data()};
  if (!in.good()) {
    return absl::NotFoundError(absl::StrCat("unable to open ", path));
  }
  return in;
}

absl::optional<std::string> ReadLine(std::ifstream& in) {
  if (!in.good()) {
    return absl::nullopt;
  }
  std::string line;
  std::getline(in, line);
  return line;
}

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
