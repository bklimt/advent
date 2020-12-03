
#include <cstdio>
#include <cstdlib>
#include <string>
#include <vector>

using std::string;
using std::vector;

void ReadMap(const char *path, vector<string> *map) {
  FILE *f = fopen(path, "r");
  if (f == nullptr) {
    printf("unable to open file");
    exit(-1);
  }
  map->push_back("");
  int c;
  while ((c = fgetc(f)) != EOF) {
    if (c == '\n') {
      map->push_back("");
    } else {
      map->back().push_back(static_cast<char>(c));
    }
  }
  if (map->back().size() == 0) {
    map->pop_back();
  }
}

void PrintMap(const vector<string> &map) {
  for (int i = 0; i < map.size(); i++) {
    printf("%s\n", map[i].c_str());
  }
}

int Sled(const vector<string> &map, int dx, int dy) {
  int t = 0;
  int j = 0;
  for (int i = 0; i < map.size(); i += dy, j += dx) {
    if (map[i][j % map[i].size()] == '#') {
      t++;
    }
  }
  printf("trees: %d\n", t);
  return t;
}

int main(int argc, char **argv) {
  vector<string> map;
  ReadMap("input.txt", &map);
  PrintMap(map);

  int64_t ans = 1;
  ans *= Sled(map, 1, 1);
  ans *= Sled(map, 3, 1);
  ans *= Sled(map, 5, 1);
  ans *= Sled(map, 7, 1);
  ans *= Sled(map, 1, 2);

  printf("ans: %lld\n", ans);

  return 0;
}
