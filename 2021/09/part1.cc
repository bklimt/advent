
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_split.h"

class DayNine {
 public:
  void Print() const;
  absl::Status Read(std::ifstream& in);
  int PartOne();
  int FloodFill(int row, int column, int basin);
  absl::StatusOr<int> PartTwo();

 private:
  std::vector<std::vector<int>> map_;
  std::vector<std::vector<int>> visited_;
};

void DayNine::Print() const {
  for (auto& line : map_) {
    for (auto& c : line) {
      std::cout << c;
    }
    std::cout << std::endl;
  }
  std::cout << std::endl;
  for (auto& line : visited_) {
    for (auto& c : line) {
      std::cout << c;
    }
    std::cout << std::endl;
  }
}

absl::Status DayNine::Read(std::ifstream& in) {
  map_.clear();
  auto line = ReadLine(in);
  while (line) {
    if (*line != "") {
      std::vector<int> row;
      for (char c : *line) {
        if (c < '0' || c > '9') {
          return absl::InternalError(
              absl::StrCat("invalid character: ", static_cast<int>(c)));
        }
        int n = c - '0';
        row.push_back(n);
      }
      visited_.emplace_back(std::vector<int>(row.size(), 0));
      map_.emplace_back(std::move(row));
    }
    line = ReadLine(in);
  }
  return absl::OkStatus();
}

int DayNine::PartOne() {
  int ans = 0;
  for (int i = 0; i < static_cast<int>(map_.size()); i++) {
    for (int j = 0; j < static_cast<int>(map_[i].size()); j++) {
      bool lowest = true;
      if (i > 0 && map_[i - 1][j] <= map_[i][j]) {
        lowest = false;
      }
      if (i < static_cast<int>(map_.size()) - 1 &&
          map_[i + 1][j] <= map_[i][j]) {
        lowest = false;
      }
      if (j > 0 && map_[i][j - 1] <= map_[i][j]) {
        lowest = false;
      }
      if (j < static_cast<int>(map_[i].size()) - 1 &&
          map_[i][j + 1] <= map_[i][j]) {
        lowest = false;
      }
      if (lowest) {
        ans += (map_[i][j] + 1);
      }
    }
  }
  return ans;
}

int DayNine::FloodFill(int row, int column, int basin) {
  if (row < 0 || row >= static_cast<int>(map_.size())) {
    return 0;
  }
  if (column < 0 || column >= static_cast<int>(map_[row].size())) {
    return 0;
  }
  if (map_[row][column] == 9) {
    return 0;
  }
  if (visited_[row][column] != 0) {
    return 0;
  }

  visited_[row][column] = basin;
  int size = 1;
  size += FloodFill(row - 1, column, basin);
  size += FloodFill(row + 1, column, basin);
  size += FloodFill(row, column - 1, basin);
  size += FloodFill(row, column + 1, basin);
  return size;
}

absl::StatusOr<int> DayNine::PartTwo() {
  std::vector<int> sizes;
  for (int i = 0; i < static_cast<int>(map_.size()); i++) {
    for (int j = 0; j < static_cast<int>(map_[i].size()); j++) {
      int size = FloodFill(i, j, sizes.size() + 1);
      if (size != 0) {
        sizes.push_back(size);
      }
    }
  }
  if (sizes.size() < 3) {
    return absl::InternalError("not enough basins");
  }
  std::sort(sizes.begin(), sizes.end());
  int end = sizes.size() - 1;
  return sizes[end] * sizes[end - 1] * sizes[end - 2];
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/09/input.txt"));

  DayNine day9;
  RETURN_IF_ERROR(day9.Read(in));

  std::cout << "Part 1: " << day9.PartOne() << std::endl;

  ASSIGN_OR_RETURN(int part2, day9.PartTwo());
  std::cout << "Part 2: " << part2 << std::endl;

  // std::cout << std::endl;
  // day9.Print();

  return absl::OkStatus();
}
