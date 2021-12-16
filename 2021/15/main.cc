
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <queue>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_split.h"

class Grid {
 public:
  Grid() {}

  Grid(const Grid&) = delete;
  Grid(Grid&&) = default;
  Grid& operator=(const Grid&) = delete;
  Grid& operator=(Grid&&) = default;

  absl::Status Read(std::ifstream& in);
  void Print() const;

 private:
  // The input grid.
  std::vector<std::vector<int>> risk_;

  // The distance from 0, 0 to each coordinate.
  std::vector<std::vector<int>> dist_;
};

absl::Status Grid::Read(std::ifstream& in) {
  std::cout << "Reading input..." << std::endl;
  auto line = ReadLine(in);
  while (line) {
    if (*line != "") {
      if (!risk_.empty()) {
        if (line->size() != risk_[0].size()) {
          return absl::InternalError(
              absl::StrFormat("inconsistent line length: %d vs %d",
                              line->size(), risk_[0].size()));
        }
      }

      std::vector<int> row;
      for (char c : *line) {
        if (c < '0' || c > '9') {
          return absl::InternalError(
              absl::StrFormat("invalid character: %c", c));
        }
        row.push_back(c - '0');
      }
      dist_.emplace_back(std::vector<int>(row.size(), 0));
      risk_.emplace_back(std::move(row));
    }
    line = ReadLine(in);
  }

  if (risk_.empty()) {
    return absl::InternalError("no data");
  }
  if (risk_[0].size() != risk_.size()) {
    return absl::InternalError(
        absl::StrFormat("not square: %d vs %d", risk_[0].size(), risk_.size()));
  }

  const int size = static_cast<int>(risk_.size());
  const int infinity = size * size * 10 + 1;

  // Initialize the distances.
  std::vector<std::pair<int, int>> queue;

  std::cout << "Initializing distances..." << std::endl;
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      queue.emplace_back(std::make_pair(i, j));
      dist_[i][j] = infinity;
    }
  }
  dist_[0][0] = 0;

  auto by_dist = [&](const std::pair<int, int>& lhs,
                     const std::pair<int, int>& rhs) {
    return dist_[lhs.first][lhs.second] > dist_[rhs.first][rhs.second];
  };

  std::sort(queue.begin(), queue.end(), by_dist);
  while (!queue.empty()) {
    std::pair<int, int> current = queue.back();
    queue.pop_back();

    std::cout << "Considering " << current.first << ", " << current.second
              << "..." << std::endl;

    if (current.first == (size - 1) && current.second == (size - 1)) {
      break;
    }

    bool needs_sort = false;

    if (current.first > 0) {
      int new_dist = dist_[current.first][current.second] +
                     risk_[current.first - 1][current.second];
      if (new_dist < dist_[current.first - 1][current.second]) {
        dist_[current.first - 1][current.second] = new_dist;
        needs_sort = true;
      }
    }
    if (current.second > 0) {
      int new_dist = dist_[current.first][current.second] +
                     risk_[current.first][current.second - 1];
      if (new_dist < dist_[current.first][current.second - 1]) {
        dist_[current.first][current.second - 1] = new_dist;
        needs_sort = true;
      }
    }
    if (current.first < size - 1) {
      int new_dist = dist_[current.first][current.second] +
                     risk_[current.first + 1][current.second];
      if (new_dist < dist_[current.first + 1][current.second]) {
        dist_[current.first + 1][current.second] = new_dist;
        needs_sort = true;
      }
    }
    if (current.second < size - 1) {
      int new_dist = dist_[current.first][current.second] +
                     risk_[current.first][current.second + 1];
      if (new_dist < dist_[current.first][current.second + 1]) {
        dist_[current.first][current.second + 1] = new_dist;
        needs_sort = true;
      }
    }

    if (needs_sort) {
      std::sort(queue.begin(), queue.end(), by_dist);
    }
  }

  std::cout << "Shortest risk: " << dist_[size - 1][size - 1] << std::endl;

  return absl::OkStatus();
}

void Grid::Print() const {
  for (const auto& row : risk_) {
    for (int n : row) {
      std::cout << (n + '0');
    }
    std::cout << std::endl;
  }
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/15/input.txt"));

  Grid grid;
  RETURN_IF_ERROR(grid.Read(in));
  // grid.Print();

  std::cout << std::endl;

  return absl::OkStatus();
}
