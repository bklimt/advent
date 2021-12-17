
cc_binary(
  name = 'main',
  srcs = ['main.cc'],
  deps = [
    "@com_google_absl//absl/strings",
    "@com_google_absl//absl/status:status",
    "@com_google_absl//absl/status:statusor",
    "//2021/base:main",
    "//2021/base:util",
  ],
  data = ["input.txt"],
)
