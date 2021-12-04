
package(default_visibility = ["//2021:__subpackages__"])

cc_library(
  name = "util",
  srcs = ["util.cc"],
  hdrs = ["util.h"],
  deps = [
    "@com_google_absl//absl/status:status",
    "@com_google_absl//absl/status:statusor",
    "@com_google_absl//absl/types:optional",
  ]
)

cc_library(
  name = "main",
  srcs = ["main.cc"],
  deps = [
    "@com_google_absl//absl/status:status",
  ]
)
