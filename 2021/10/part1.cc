
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"

absl::StatusOr<std::pair<int, int64_t>> ScoreLine(absl::string_view s) {
  std::vector<char> stack;

#define SWITCH_CASE(open, close, score)                                  \
  case open:                                                             \
    stack.push_back(open);                                               \
    break;                                                               \
  case close:                                                            \
    if (stack.back() != open) {                                          \
      std::cout << s << " - Expected " << stack.back() << ", but found " \
                << close << " instead." << std::endl;                    \
      return std::make_pair(score, 0);                                   \
    }                                                                    \
    stack.pop_back();                                                    \
    break

  for (char c : s) {
    switch (c) {
      SWITCH_CASE('(', ')', 3);
      SWITCH_CASE('[', ']', 57);
      SWITCH_CASE('{', '}', 1197);
      SWITCH_CASE('<', '>', 25137);
      default:
        return absl::InternalError(
            absl::StrCat("invalid character: ", static_cast<int>(c)));
    }
  }
#undef SWITCH_CASE

  int64_t part2 = 0;
  while (!stack.empty()) {
    part2 *= 5;
    char c = stack.back();
    switch (c) {
      case '(':
        part2 += 1;
        break;
      case '[':
        part2 += 2;
        break;
      case '{':
        part2 += 3;
        break;
      case '<':
        part2 += 4;
        break;
    }
    stack.pop_back();
  }

  std::cout << s << " - " << part2 << " total points" << std::endl;

  return std::make_pair(0, part2);
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/10/input.txt"));

  int part1 = 0;
  std::vector<int64_t> part2s;

  auto line = ReadLine(in);
  while (line) {
    if (*line != "") {
      ASSIGN_OR_RETURN(auto scores, ScoreLine(*line));
      part1 += scores.first;
      if (scores.second != 0) {
        part2s.push_back(scores.second);
      }
    }
    line = ReadLine(in);
  }

  std::sort(part2s.begin(), part2s.end());
  int64_t part2 = part2s[part2s.size() / 2];

  std::cout << "Part 1: " << part1 << std::endl;
  std::cout << "Part 2: " << part2 << std::endl;

  // 70880594 is too low.

  return absl::OkStatus();
}
