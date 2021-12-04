
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>

#include "2021/base/util.h"
#include "absl/status/status.h"

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/04/input.txt"));

  auto line1 = ReadLine(in);
  if (!line1) {
    return absl::InvalidArgumentError("expected list of numbers");
  }

  std::cout << "line1: " << *line1;

  return absl::OkStatus();
}
