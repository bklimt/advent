
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_split.h"

using IntMap = std::vector<std::vector<int>>;

void PrintMap(const IntMap& map) {
  for (auto& line : map) {
    for (auto& c : line) {
      std::cout << c;
    }
    std::cout << std::endl;
  }
}

absl::StatusOr<IntMap> ReadMap(std::ifstream& in) {
  IntMap map;
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
      map.emplace_back(std::move(row));
    }
    line = ReadLine(in);
  }
  return map;
}

int PartOne(const IntMap& map) {
  int ans = 0;
  for (int i = 0; i < map.size(); i++) {
    for (int j = 0; j < map[i].size(); j++) {
      bool lowest = true;
      if (i > 0 && map[i - 1][j] <= map[i][j]) {
        lowest = false;
      }
      if (i < map.size() - 1 && map[i + 1][j] <= map[i][j]) {
        lowest = false;
      }
      if (j > 0 && map[i][j - 1] <= map[i][j]) {
        lowest = false;
      }
      if (j < map[i].size() - 1 && map[i][j + 1] <= map[i][j]) {
        lowest = false;
      }
      if (lowest) {
        ans += (map[i][j] + 1);
      }
    }
  }
  return ans;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/09/input.txt"));
  ASSIGN_OR_RETURN(auto map, ReadMap(in));
  PrintMap(map);

  std::cout << "Part 1: " << PartOne(map) << std::endl;

  return absl::OkStatus();
}
