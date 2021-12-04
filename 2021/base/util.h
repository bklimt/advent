#ifndef __BASE_UTIL_H__
#define __BASE_UTIL_H__

#include "absl/status/statusor.h"
#include "absl/types/optional.h"

#define RETURN_IF_ERROR(val) \
  do {                       \
    auto status = (val);     \
    if (!status.ok()) {      \
      return status;         \
    }                        \
  } while (false)

#define ASSIGN_OR_RETURN_INTERNAL(var, val, status_or) \
  auto status_or = (val);                              \
  if (!status_or.ok()) {                               \
    return status_or.status();                         \
  }                                                    \
  var = *std::move(status_or)

#define ASSIGN_OR_RETURN(var, val) \
  ASSIGN_OR_RETURN_INTERNAL(var, val, status_or_##__LINE__)

absl::StatusOr<std::ifstream> OpenFile(absl::string_view path);

absl::optional<std::string> ReadLine(std::ifstream& in);

#endif  // __BASE_UTIL_H__
