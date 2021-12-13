
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"

class DayEleven {
 public:
  absl::Status ReadFile(std::ifstream& in) {
    auto line = ReadLine(in);
    while (line) {
      if (*line != "") {
        std::vector<int> row;
        for (char c : *line) {
          if (c < '0' || c > '9') {
            return absl::InternalError(
                absl::StrFormat("invalid character: %c", c));
          }
          row.push_back(c - '0');
        }
        if (row.size() != 10) {
          return absl::InternalError(
              absl::StrFormat("invalid line length: %s %d", *line, row.size()));
        }
        flash_.emplace_back(std::vector<bool>(row.size(), false));
        grid_.emplace_back(std::move(row));
      }
      line = ReadLine(in);
    }
    if (grid_.size() != 10) {
      return absl::InternalError(
          absl::StrFormat("invalid number of rows: %d", grid_.size()));
    }
    return absl::OkStatus();
  }

  void Print() const {
    for (auto& row : grid_) {
      for (auto c : row) {
        std::cout << c;
      }
      std::cout << std::endl;
    }
  }

  int Step() {
    // Increment everything by 1.
    for (int i = 0; i < 10; i++) {
      for (int j = 0; j < 10; j++) {
        grid_[i][j]++;
      }
    }

    bool flashed = true;
    while (flashed) {
      flashed = false;
      for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
          if (flash_[i][j]) {
            continue;
          }
          if (grid_[i][j] > 9) {
            flashed = true;
            flash_[i][j] = true;
            Increment(i - 1, j - 1);
            Increment(i - 1, j);
            Increment(i - 1, j + 1);
            Increment(i, j - 1);
            Increment(i, j + 1);
            Increment(i + 1, j - 1);
            Increment(i + 1, j);
            Increment(i + 1, j + 1);
          }
        }
      }
    }

    return CountAndClear();
  }

  void Increment(int row, int column) {
    if (row < 0 || column < 0 || row > 9 || column > 9) {
      return;
    }
    grid_[row][column]++;
  }

  int CountAndClear() {
    int flashes = 0;
    for (int i = 0; i < 10; i++) {
      for (int j = 0; j < 10; j++) {
        if (flash_[i][j]) {
          flashes++;
          flash_[i][j] = false;
        }
        if (grid_[i][j] > 9) {
          grid_[i][j] = 0;
        }
      }
    }
    return flashes;
  }

 private:
  std::vector<std::vector<int>> grid_;
  std::vector<std::vector<bool>> flash_;
};

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/11/input.txt"));

  DayEleven day11;
  RETURN_IF_ERROR(day11.ReadFile(in));
  day11.Print();

  int total_flashes = 0;

  for (int i = 1; true; i++) {
    int flashes = day11.Step();
    total_flashes += flashes;
    if (flashes == 100) {
      std::cout << "Part 2: " << i << std::endl;
      return absl::OkStatus();
    }
    // std::cout << std::endl;
    // std::cout << "After " << i << " steps:" << std::endl;
    // day11.Print();
    // std::cout << "Flashes: " << flashes << std::endl;

    if (i == 100) {
      std::cout << "Part 1: " << total_flashes << std::endl;
    }
  }
}
