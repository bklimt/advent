
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_split.h"

class Paper {
 public:
  Paper() : max_x_(0), max_y_(0) {}

  Paper(const Paper&) = delete;
  Paper(Paper&&) = default;
  Paper& operator=(const Paper&) = delete;
  Paper& operator=(Paper&&) = default;

  absl::Status Read(std::ifstream& in);
  void Print() const;

 private:
  int max_x_;
  int max_y_;
  std::set<std::pair<int, int>> dots_;
};

absl::Status Paper::Read(std::ifstream& in) {
  auto line = ReadLine(in);
  while (line) {
    if (*line == "") {
      return absl::OkStatus();
    }

    ASSIGN_OR_RETURN(auto numbers, ParseNumbers(*line));
    if (numbers.size() != 2) {
      return absl::InternalError(absl::StrCat("invalid line: ", *line));
    }

    dots_.insert(std::make_pair(numbers[0], numbers[1]));
    max_x_ = std::max(numbers[0], max_x_);
    max_y_ = std::max(numbers[1], max_y_);

    line = ReadLine(in);
  }
  return absl::OkStatus();
}

void Paper::Print() const {
  for (int y = 0; y <= max_y_; y++) {
    for (int x = 0; x <= max_x_; x++) {
      if (dots_.find(std::make_pair(x, y)) != dots_.end()) {
        std::cout << "#";
      } else {
        std::cout << ".";
      }
    }
    std::cout << std::endl;
  }
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/13/input.txt"));

  Paper paper;
  RETURN_IF_ERROR(paper.Read(in));
  paper.Print();

  return absl::OkStatus();
}
