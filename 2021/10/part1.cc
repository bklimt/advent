
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"

absl::StatusOr<int> ScoreLine(absl::string_view s) {
  std::vector<char> stack;

#define SWITCH_CASE(open, close, score)                                  \
  case open:                                                             \
    stack.push_back(open);                                               \
    break;                                                               \
  case close:                                                            \
    if (stack.back() != open) {                                          \
      std::cout << s << " - Expected " << stack.back() << ", but found " \
                << close << " instead." << std::endl;                    \
      return score;                                                      \
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
  return 0;
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/10/input.txt"));

  int part1 = 0;

  auto line = ReadLine(in);
  while (line) {
    if (*line != "") {
      ASSIGN_OR_RETURN(int score, ScoreLine(*line));
      part1 += score;
    }
    line = ReadLine(in);
  }

  std::cout << "Part 1: " << part1 << std::endl;

  return absl::OkStatus();
}
