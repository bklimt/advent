
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

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

  // Map from row, column in input to a node index.
  std::map<std::pair<int, int>, int> coord_to_index_;

  // Map from node index to row, column.
  std::vector<std::pair<int, int>> index_to_coord_;

  // dist_[i][j] is the shortest distance from node i to node j.
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

  int size = static_cast<int>(risk_.size());

  // Create notes out of them.
  std::cout << "Creating index..." << std::endl;
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      coord_to_index_[std::make_pair(i, j)] = index_to_coord_.size();
      index_to_coord_.push_back(std::make_pair(i, j));
    }
  }

  int nodes = static_cast<int>(index_to_coord_.size());

  // Compute the initial distances.
  for (int i = 0; i < nodes; i++) {
    dist_.emplace_back(std::vector<int>(nodes, 0));
  }
  std::cout << "Computing initial distances..." << std::endl;
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      int start = coord_to_index_[std::make_pair(i, j)];
      // Up.
      if (i > 0) {
        dist_[start][coord_to_index_[std::make_pair(i - 1, j)]] =
            risk_[i - 1][j];
      }
      if (i < size - 1) {
        dist_[start][coord_to_index_[std::make_pair(i + 1, j)]] =
            risk_[i + 1][j];
      }
      if (j > 0) {
        dist_[start][coord_to_index_[std::make_pair(i, j - 1)]] =
            risk_[i][j - 1];
      }
      if (j < size - 1) {
        dist_[start][coord_to_index_[std::make_pair(i, j + 1)]] =
            risk_[i][j + 1];
      }
    }
  }

  std::cout << "Finding shorter paths..." << std::endl;
  for (int k = 0; k < nodes; k++) {
    double progress = static_cast<double>(k) / nodes;
    progress *= 100;
    std::cout << progress << "%: Considering node " << k << " of " << nodes
              << std::endl;
    for (int i = 0; i < nodes; i++) {
      for (int j = 0; j < nodes; j++) {
        if (i == j || i == k || j == k) {
          continue;
        }
        if (dist_[i][k] + dist_[k][j] < dist_[i][j]) {
          dist_[i][j] = dist_[i][k] + dist_[k][j];
        }
      }
    }
  }

  std::cout << "Finished finding paths." << std::endl;

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
  grid.Print();

  std::cout << std::endl;

  return absl::OkStatus();
}
