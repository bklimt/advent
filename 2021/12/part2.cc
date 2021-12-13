
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>

#include "2021/base/util.h"
#include "absl/cleanup/cleanup.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_join.h"
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

  void AddAdjacent(int other) { adjacent_.insert(other); }

  const std::set<int>& adjacent() const { return adjacent_; }

 private:
  std::string name_;
  int index_;
  bool big_;
  std::set<int> adjacent_;
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
        caves_[a].AddAdjacent(b);
        if (parts[0] != "start" && parts[1] != "end") {
          caves_[b].AddAdjacent(a);
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
      for (int other : cave.adjacent()) {
        std::cout << cave.name() << "-" << caves_[other].name() << std::endl;
      }
    }
    std::cout << std::endl;
  }

  int CountPaths(int start, int end, std::vector<int>* visited, bool free_pass,
                 std::vector<std::string>* path,
                 std::vector<std::string>* all_paths) const {
    path->push_back(caves_[start].name());
    auto cleanup = absl::MakeCleanup([path]() { path->pop_back(); });
    if (start == end) {
      all_paths->push_back(absl::StrJoin(*path, ","));
      return 1;
    }
    if ((*visited)[start] > 0) {
      if (!free_pass) {
        return 0;
      }
      free_pass = false;
    }
    if (!caves_[start].big()) {
      (*visited)[start]++;
    }

    int paths = 0;
    for (int other : caves_[start].adjacent()) {
      paths += CountPaths(other, end, visited, free_pass, path, all_paths);
    }

    if (!caves_[start].big()) {
      (*visited)[start]--;
    }
    return paths;
  }

  int CountAllPaths(std::vector<std::string>* all_paths) {
    std::vector<int> visited(caves_.size(), 0);
    int start = name_index_["start"];
    int end = name_index_["end"];
    std::vector<std::string> path;
    return CountPaths(start, end, &visited, true, &path, all_paths);
  }

 private:
  std::vector<Cave> caves_;
  std::map<std::string, int> name_index_;
};

absl::Status Main() {
  ASSIGN_OR_RETURN(auto in, OpenFile("2021/12/input.txt"));

  Graph graph;
  RETURN_IF_ERROR(graph.ReadFile(in));
  graph.Print();

  std::vector<std::string> all_paths;
  int count = graph.CountAllPaths(&all_paths);
  std::sort(all_paths.begin(), all_paths.end());
  for (const std::string& path : all_paths) {
    std::cout << path << std::endl;
  }

  std::cout << "Total: " << count << std::endl;

  return absl::OkStatus();
}
