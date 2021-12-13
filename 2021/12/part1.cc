
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_split.h"

class Cave {
 public:
  Cave(absl::string_view name, int index)
      : name_(name), index_(index), big_(isupper(name[0])) {}

  Cave(const Cave&) = delete;
  Cave& operator=(const Cave&) = delete;

  Cave(Cave&&) = default;
  Cave& operator=(Cave&&) = default;

  const std::string& name() const { return name_; }
  int index() const { return index_; }
  bool big() const { return big_; }

  std::string ToString() const {
    return absl::StrFormat("%s{%d, %s}", name_, index_,
                           (big_ ? "true" : "false"));
  }

 private:
  std::string name_;
  int index_;
  bool big_;
};

class Graph {
 public:
  absl::Status ReadFile(std::ifstream& in) {
    auto line = ReadLine(in);
    while (line) {
      if (*line != "") {
        std::vector<absl::string_view> parts = absl::StrSplit(*line, "-");
        if (parts.size() != 2) {
          return absl::InternalError(absl::StrCat("invalid line: ", *line));
        }
        int a = UpsertCave(parts[0]);
        int b = UpsertCave(parts[1]);
        adjacency_.insert(std::make_pair(a, b));
        if (parts[0] != "start" && parts[1] != "end") {
          adjacency_.insert(std::make_pair(b, a));
        }
      }
      line = ReadLine(in);
    }
    return absl::OkStatus();
  }

  int UpsertCave(absl::string_view name) {
    return UpsertCave(std::string(name));
  }

  int UpsertCave(const std::string& name) {
    auto it = name_index_.find(name);
    if (it != name_index_.end()) {
      return it->second;
    }
    return AddCave(name);
  }

  int AddCave(const std::string& name) {
    int index = caves_.size();
    name_index_[name] = index;
    caves_.push_back(Cave(name, index));
    return index;
  }

  void Print() const {
    for (const Cave& cave : caves_) {
      std::cout << cave.ToString() << std::endl;
    }
    std::cout << std::endl;
    for (auto pair : adjacency_) {
      std::cout << caves_[pair.first].name() << "-"
                << caves_[pair.second].name() << std::endl;
    }
  }

 private:
  std::vector<Cave> caves_;
  std::map<std::string, int> name_index_;
  std::set<std::pair<int, int>> adjacency_;
};

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/12/input.txt"));

  Graph graph;
  RETURN_IF_ERROR(graph.ReadFile(in));
  graph.Print();

  return absl::OkStatus();
}
