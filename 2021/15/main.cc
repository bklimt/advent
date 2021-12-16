
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

inline int weird_mod_10(int x) { return ((x - 1) % 9) + 1; }

class Node {
 public:
  explicit Node(int risk) : risk_(risk), dist_(0), visited_(false) {}

  int risk() const { return risk_; }
  int dist() const { return dist_; }
  bool visited() const { return visited_; }

  void set_dist(int dist) { dist_ = dist; }
  void set_visited(bool visited) { visited_ = visited; }

 private:
  int risk_;
  int dist_;
  bool visited_;
};

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
  std::vector<std::vector<Node>> nodes_;
};

absl::Status Grid::Read(std::ifstream& in) {
  std::cout << "Reading input..." << std::endl;
  auto line = ReadLine(in);
  while (line) {
    if (*line != "") {
      if (!nodes_.empty()) {
        if ((line->size() * 5) != nodes_[0].size()) {
          return absl::InternalError(
              absl::StrFormat("inconsistent line length: %d vs %d",
                              line->size() * 5, nodes_[0].size()));
        }
      }

      std::vector<Node> row;
      for (char c : *line) {
        if (c < '0' || c > '9') {
          return absl::InternalError(
              absl::StrFormat("invalid character: %c", c));
        }
        row.emplace_back(Node(c - '0'));
      }

      int original_size = row.size();
      for (int i = 1; i < 5; i++) {
        for (int j = 0; j < original_size; j++) {
          row.push_back(Node(weird_mod_10(row[j].risk() + i)));
        }
      }

      nodes_.emplace_back(std::move(row));
    }
    line = ReadLine(in);
  }

  int original_size = nodes_.size();
  for (int i = 1; i < 5; i++) {
    for (int j = 0; j < original_size; j++) {
      std::vector<Node> row;
      for (auto& node : nodes_[j]) {
        row.emplace_back(Node(weird_mod_10(node.risk() + i)));
      }
      nodes_.emplace_back(std::move(row));
    }
  }

  if (nodes_.empty()) {
    return absl::InternalError("no data");
  }
  if (nodes_[0].size() != nodes_.size()) {
    return absl::InternalError(absl::StrFormat(
        "not square: %d vs %d", nodes_[0].size(), nodes_.size()));
  }

  // Print();

  const int size = static_cast<int>(nodes_.size());
  const int infinity = size * size * 10 + 1;
  std::cout << "size = " << size << std::endl;

  // Initialize the distances.
  std::vector<std::pair<int, int>> queue;

  std::cout << "Initializing distances..." << std::endl;
  for (int i = 0; i < size; i++) {
    for (int j = 0; j < size; j++) {
      nodes_[i][j].set_dist(infinity);
    }
  }
  nodes_[0][0].set_dist(0);
  queue.emplace_back(std::make_pair(0, 0));

  auto by_dist = [&](const std::pair<int, int>& lhs,
                     const std::pair<int, int>& rhs) {
    return nodes_[lhs.first][lhs.second].dist() >
           nodes_[rhs.first][rhs.second].dist();
  };

  absl::Time start = absl::Now();
  int total_nodes = size * size;

  int processed = 0;
  while (!queue.empty()) {
    std::pair<int, int> current_pos = queue.back();
    queue.pop_back();
    processed++;

    if (queue.size() % 100 == 0) {
      absl::Time now = absl::Now();
      absl::Duration elapsed = now - start;
      absl::Duration remaining_time =
          elapsed * (static_cast<double>(total_nodes - processed) / processed);
      absl::Time end_time = now + remaining_time;
      std::cout << "Remaining: " << remaining_time << " at " << end_time << " ("
                << (total_nodes - processed) << " nodes, " << queue.size()
                << " in queue)" << std::endl;
    }

    // std::cout << "Remaining: " << queue.size() << std::endl;
    // std::cout << "Considering " << current.first << ", " << current.second
    //          << "..." << std::endl;

    if (current_pos.first == (size - 1) && current_pos.second == (size - 1)) {
      break;
    }

    bool needs_sort = false;
    Node& current = nodes_[current_pos.first][current_pos.second];

#define TRY_DIR(drow, dcol)                                                    \
  do {                                                                         \
    Node& other = nodes_[current_pos.first + drow][current_pos.second + dcol]; \
    int new_dist = current.dist() + other.risk();                              \
    if (new_dist < other.dist()) {                                             \
      other.set_dist(new_dist);                                                \
      if (!other.visited()) {                                                  \
        other.set_visited(true);                                               \
        queue.push_back(std::make_pair(current_pos.first + drow,               \
                                       current_pos.second + dcol));            \
      }                                                                        \
      needs_sort = true;                                                       \
    }                                                                          \
  } while (0)

    if (current_pos.first > 0) {
      TRY_DIR(-1, 0);
    }
    if (current_pos.second > 0) {
      TRY_DIR(0, -1);
    }
    if (current_pos.first < size - 1) {
      TRY_DIR(1, 0);
    }
    if (current_pos.second < size - 1) {
      TRY_DIR(0, 1);
    }

    if (needs_sort) {
      std::sort(queue.begin(), queue.end(), by_dist);
    }
  }

  std::cout << "Shortest risk: " << nodes_[size - 1][size - 1].dist()
            << std::endl;

  return absl::OkStatus();
}

void Grid::Print() const {
  for (const auto& row : nodes_) {
    for (const Node& n : row) {
      std::cout << n.risk();
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
