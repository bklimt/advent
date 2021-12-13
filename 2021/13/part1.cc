
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

  void FoldX(int x);
  void FoldY(int y);

  int DotCount() const;

 private:
  int max_x_;
  int max_y_;
  std::set<std::pair<int, int>> dots_;
};

absl::Status Paper::Read(std::ifstream& in) {
  bool part1 = true;
  auto line = ReadLine(in);
  while (line) {
    if (*line == "") {
      part1 = false;
    } else if (part1) {
      ASSIGN_OR_RETURN(auto numbers, ParseNumbers(*line));
      if (numbers.size() != 2) {
        return absl::InternalError(absl::StrCat("invalid line: ", *line));
      }

      dots_.insert(std::make_pair(numbers[0], numbers[1]));
      max_x_ = std::max(numbers[0], max_x_);
      max_y_ = std::max(numbers[1], max_y_);
    } else {
      // Process the fold instructions.
      if (!absl::StartsWith(*line, "fold along ")) {
        return absl::InternalError(absl::StrCat("invalid line: ", *line));
      }
      std::string num_part = line->substr(13);
      int n;
      if (!absl::SimpleAtoi(num_part, &n)) {
        return absl::InternalError(absl::StrCat("invalid number: ", num_part));
      }
      if ((*line)[11] == 'x') {
        FoldX(n);
      } else if ((*line)[11] == 'y') {
        FoldY(n);
      } else {
        return absl::InternalError(absl::StrCat("invalid line: ", *line));
      }
    }

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

void Paper::FoldX(int x) {
  std::set<std::pair<int, int>> new_dots;
  for (const auto& dot : dots_) {
    if (dot.first > x) {
      new_dots.insert(std::make_pair(x - (dot.first - x), dot.second));
    } else {
      new_dots.insert(dot);
    }
  }
  dots_ = std::move(new_dots);
  max_x_ = x;
}

void Paper::FoldY(int y) {
  std::set<std::pair<int, int>> new_dots;
  for (const auto& dot : dots_) {
    if (dot.second > y) {
      new_dots.insert(std::make_pair(dot.first, y - (dot.second - y)));
    } else {
      new_dots.insert(dot);
    }
  }
  dots_ = std::move(new_dots);
  max_y_ = y;
}

int Paper::DotCount() const { return dots_.size(); }

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/13/input.txt"));

  Paper paper;
  RETURN_IF_ERROR(paper.Read(in));
  paper.Print();

  std::cout << std::endl;

  // paper.FoldX(655);
  // paper.Print();
  // std::cout << "Part 1: " << paper.DotCount() << std::endl;

  return absl::OkStatus();
}
