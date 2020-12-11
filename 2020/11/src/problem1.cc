
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

bool IsOccupied(const vector<string> &input, int i, int j) {
  if (i < 0 || j < 0 || i >= input.size() || j >= input[i].size()) {
    return false;
  }
  return input[i][j] == '#';
}

bool Pass(const vector<string> &input, vector<string>* output, int *seats) {
  output->clear();
  *seats = 0;
  bool changed = false;
  for (int i = 0; i < input.size(); i++) {
    output->push_back("");
    for (int j = 0; j < input[i].size(); j++) {
      char c = input[i][j];
      if (c != '.') {
        int people_around = 0;
        if (IsOccupied(input, i-1, j-1)) { people_around++; }
        if (IsOccupied(input, i-1, j  )) { people_around++; }
        if (IsOccupied(input, i-1, j+1)) { people_around++; }
        if (IsOccupied(input, i  , j-1)) { people_around++; }
        if (IsOccupied(input, i  , j+1)) { people_around++; }
        if (IsOccupied(input, i+1, j-1)) { people_around++; }
        if (IsOccupied(input, i+1, j  )) { people_around++; }
        if (IsOccupied(input, i+1, j+1)) { people_around++; }
        if (c == 'L' && people_around == 0) {
          c = '#';
          changed = true;
        } else if (c == '#' && people_around >= 4) {
          c = 'L';
          changed = true;
        }
      }
      output->back().push_back(c);
      if (c == '#') {
        (*seats)++;
      }
    }
  }
  return changed;
}

int main(int argc, char **argv) {
  vector<string> map1;
  vector<string> map2;

  vector<string> *current = &map1;
  vector<string> *next = &map2;

  ReadMap("input.txt", current);
  PrintMap(*current);
  printf("\n");

  int seats;
  while (true) {
    if (!Pass(*current, next, &seats)) {
      break;
    }
    vector<string> *temp = current;
    current = next;
    next = temp;

    PrintMap(*current);
    printf("\n");
  }

  printf("seats: %d\n", seats);

  return 0;
}
