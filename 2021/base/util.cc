
#include "2021/base/util.h"

#include <fstream>

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
