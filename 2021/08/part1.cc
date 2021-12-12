
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_split.h"

std::string SortString(absl::string_view input) {
  std::string output(input);
  std::sort(output.begin(), output.end());
  return output;
}

absl::StatusOr<char> SegmentDiff(const std::string& a, const std::string& b) {
  if (a.size() > b.size()) {
    return SegmentDiff(b, a);
  }

  char r = '\0';
  for (auto c : b) {
    if (a.find(c) == std::string::npos) {
      if (r != '\0') {
        return absl::InternalError(
            absl::StrCat("diff between ", a, " and ", b, " has multiple"));
      }
      r = c;
    }
  }

  if (r == '\0') {
    return absl::InternalError(
        absl::StrCat("diff between ", a, " and ", b, " has none"));
  }
  return r;
}

absl::StatusOr<std::map<std::string, int>> DecodeDisplays(
    const std::vector<std::string>& inputs) {
#define SET_VAL(name)                              \
  if (name >= 0) {                                 \
    return absl::InternalError("multiple " #name); \
  }                                                \
  name = i

#define CHECK_VAL(name)                                 \
  do {                                                  \
    if (name == -1) {                                   \
      return absl::InternalError("didn't find " #name); \
    }                                                   \
  } while (0)

  // Figure out which one are 1, 4, 7, and 8.
  int one = -1;
  int four = -1;
  int seven = -1;
  int eight = -1;
  for (int i = 0; i < static_cast<int>(inputs.size()); i++) {
    if (inputs[i].size() == 2) {
      SET_VAL(one);
    }
    if (inputs[i].size() == 3) {
      SET_VAL(seven);
    }
    if (inputs[i].size() == 4) {
      SET_VAL(four);
    }
    if (inputs[i].size() == 7) {
      SET_VAL(eight);
    }
  }
  CHECK_VAL(one);
  CHECK_VAL(four);
  CHECK_VAL(seven);
  CHECK_VAL(eight);

  // Figure out 6.
  int six = -1;
  // segment c is the part of 1 that's not in 6.
  char segment_c = '\0';
  // segment f is the part of 1 that's also in 6.
  char segment_f = '\0';
  for (int i = 0; i < static_cast<int>(inputs.size()); i++) {
    if (inputs[i].size() != 6) {
      continue;
    }
    if (inputs[i].find(inputs[one][0]) == std::string::npos ||
        inputs[i].find(inputs[one][1]) == std::string::npos) {
      SET_VAL(six);
      if (inputs[i].find(inputs[one][0]) == std::string::npos) {
        segment_c = inputs[one][0];
        segment_f = inputs[one][1];
      } else {
        segment_c = inputs[one][1];
        segment_f = inputs[one][0];
      }
    }
  }
  CHECK_VAL(six);

  // Figure out 2, 3, and 5.
  int two = -1;
  int three = -1;
  int five = -1;
  for (int i = 0; i < static_cast<int>(inputs.size()); i++) {
    if (inputs[i].size() != 5) {
      continue;
    }
    bool has_c = (inputs[i].find(segment_c) != std::string::npos);
    bool has_f = (inputs[i].find(segment_f) != std::string::npos);
    if (has_c && has_f) {
      SET_VAL(three);
    } else if (has_c) {
      SET_VAL(two);
    } else if (has_f) {
      SET_VAL(five);
    } else {
      return absl::InternalError("five segment input is invalid");
    }
  }
  CHECK_VAL(two);
  CHECK_VAL(three);
  CHECK_VAL(five);

  ASSIGN_OR_RETURN(char segment_e, SegmentDiff(inputs[six], inputs[five]));

  // Figure out zero and nine.
  int zero = -1;
  int nine = -1;
  for (int i = 0; i < static_cast<int>(inputs.size()); i++) {
    if (inputs[i].size() != 6) {
      continue;
    }
    if (i == six) {
      continue;
    }
    if (inputs[i].find(segment_e) != std::string::npos) {
      SET_VAL(zero);
    } else {
      SET_VAL(nine);
    }
  }
  CHECK_VAL(zero);
  CHECK_VAL(nine);

  std::map<std::string, int> result;
  result[inputs[zero]] = 0;
  result[inputs[one]] = 1;
  result[inputs[two]] = 2;
  result[inputs[three]] = 3;
  result[inputs[four]] = 4;
  result[inputs[five]] = 5;
  result[inputs[six]] = 6;
  result[inputs[seven]] = 7;
  result[inputs[eight]] = 8;
  result[inputs[nine]] = 9;

  return result;
}

absl::StatusOr<std::map<std::string, int>> DecodeDisplayString(
    absl::string_view input) {
  std::vector<absl::string_view> parts = absl::StrSplit(input, " ");

  std::vector<std::string> inputs;
  for (auto part : parts) {
    inputs.push_back(SortString(part));
  }

  return DecodeDisplays(inputs);
}

// The pair is part1, part2.
absl::StatusOr<std::pair<int, int>> DecodeLine(const std::string& line) {
  std::vector<absl::string_view> parts = absl::StrSplit(line, " | ");
  if (parts.size() != 2) {
    return absl::InternalError(absl::StrCat("invalid line: ", line));
  }

  ASSIGN_OR_RETURN(auto lookup, DecodeDisplayString(parts[0]));

  /*
  for (auto pair : lookup) {
    std::cout << pair.first << ": " << pair.second << std::endl;
  }
  */

  std::vector<absl::string_view> digits = absl::StrSplit(parts[1], " ");

  int part1 = 0;
  int part2 = 0;
  for (auto digit : digits) {
    std::string normalized = SortString(digit);
    auto it = lookup.find(normalized);
    if (it == lookup.end()) {
      return absl::InternalError(absl::StrCat("unknown string: ", normalized));
    }
    int n = it->second;
    if (n == 1 || n == 4 || n == 7 || n == 8) {
      part1++;
    }
    part2 = (part2 * 10) + n;
  }

  return std::make_pair(part1, part2);
}

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/08/input.txt"));

  int part1 = 0;

  auto line = ReadLine(in);
  while (line) {
    if (*line != "") {
      ASSIGN_OR_RETURN(auto pair, DecodeLine(*line));
      part1 += pair.first;
    }
    line = ReadLine(in);
  }

  std::cout << part1 << std::endl;

  return absl::OkStatus();
}
