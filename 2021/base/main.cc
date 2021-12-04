
#include <iostream>

#include "absl/status/status.h"

absl::Status Main();

int main(int argc, char **argv) {
  auto status = Main();
  if (!status.ok()) {
    std::cerr << "error: " << status;
    return -1;
  }
  return 0;
}
