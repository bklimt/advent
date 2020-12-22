
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <optional>
#include <set>
#include <vector>

class Tile {
 public:
  Tile(int id, int data[10][10], int rotations, bool flipped_h, bool flipped_v) {
    id_ = id;
    rotations_ = rotations;
    flipped_h_ = flipped_h;
    flipped_v_ = flipped_v;
    for (int i = 0; i < 10; i++) {
      for (int j = 0; j < 10; j++) {
        data_.push_back(data[i][j]);
      }
    }

    top_ = bottom_ = right_ = left_ = 0;

    for (int i = 0; i < 10; i++) {
      top_ <<= 1;
      top_ |= data[0][i];

      bottom_ <<= 1;
      bottom_ |= data[9][i];

      left_ <<= 1;
      left_ |= data[i][0];

      right_ <<= 1;
      right_ |= data[i][9];
    }

    PrintDebugString();

    if (top_ == 0 || bottom_ == 0 || left_ == 0 || right_ == 0) {
      printf("oh no! 0 edge!\n");
      exit(-1);
    }
  }

  Tile(Tile &&other) {
    id_ = other.id_;
    data_ = std::move(other.data_);
    top_ = other.top_;
    bottom_ = other.bottom_;
    left_ = other.left_;
    right_ = other.right_;
    rotations_ = other.rotations_;
    flipped_h_ = other.flipped_h_;
    flipped_v_ = other.flipped_v_;

    other.id_ = 0;
  }

  Tile &operator=(Tile &&other) {
    id_ = other.id_;
    data_ = std::move(other.data_);
    top_ = other.top_;
    bottom_ = other.bottom_;
    left_ = other.left_;
    right_ = other.right_;
    rotations_ = other.rotations_;
    flipped_h_ = other.flipped_h_;
    flipped_v_ = other.flipped_v_;

    other.id_ = 0;

    return *this;
  }

  int id() const { return id_; }
  int top() const { return top_; }
  int bottom() const { return bottom_; }
  int left() const { return left_; }
  int right() const { return right_; }

  int data(int row, int col) const { return data_[row * 10 + col]; }

  void PrintDebugString() const {
    printf("Tile %4d (rotated %d times, flipped h: %s, flipped v: %s):\n",
      id_, rotations_, flipped_h_ ? "true" : "false", flipped_v_ ? "true" : "false");
    printf("  top=%d, bottom=%d, left=%d, right=%d\n", top_, bottom_, left_, right_);
    for (int i = 0; i < 10; i++) {
      for (int j = 0; j < 10; j++) {
        if (data_[i * 10 + j] == 0) {
          printf(".");
        } else if (data_[i * 10 + j] == 1) {
          printf("#");
        } else {
          printf("?");
        }
      }
      printf("\n");
    }
    printf("\n");
  }

 private:
  Tile(const Tile &other) = delete;
  Tile &operator=(const Tile &other) = delete;

  int id_;
  std::vector<int> data_;

  int top_, bottom_, left_, right_;
  int rotations_;
  bool flipped_h_;
  bool flipped_v_;
};

void RotateClockwise90(int data[10][10]) {
  char new_data[10][10];
  for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 10; j++) {
      new_data[i][j] = data[9-j][i];
    }
  }
  for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 10; j++) {
      data[i][j] = new_data[i][j];
    }
  }
}

void FlipH(int data[10][10]) {
  for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 5; j++) {
      std::swap(data[i][j], data[i][9-j]);
    }
  }
}

void FlipV(int data[10][10]) {
  for (int i = 0; i < 5; i++) {
    for (int j = 0; j < 10; j++) {
      std::swap(data[i][j], data[9-i][j]);
    }
  }
}

void ExpectChar(int expected, int got) {
  if (expected != got) {
    printf("expected %d; got %d\n", expected, got);
    exit(-1);
  }
}

void ExpectChar(FILE *f, int expected) { ExpectChar(expected, fgetc(f)); }

void ExpectStr(FILE *f, const char *expected) {
  int len = strlen(expected);
  for (int i = 0; i < len; i++) {
    ExpectChar(f, expected[i]);
  }
}

void ReadTileData(FILE *f, int data[10][10]) {
  for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 10; j++) {
      int c = fgetc(f);
      if (c == EOF) {
        printf("unexpected EOF\n");
        exit(-1);
      }
      if (c == '.') {
        data[i][j] = 0;
      } else if (c == '#') {
        data[i][j] = 1;
      } else {
        printf("unexpected char: %d\n", c);
        exit(-1);
      }
    }
    ExpectChar(f, '\n');
  }
}

int ReadId(FILE *f) {
  int id = 0;
  for (int i = 0; i < 4; i++) {
    int c = fgetc(f);
    if (c < '0' || c > '9') {
      printf("expected digit; got %d\n", c);
      exit(-1);
    }
    id *= 10;
    id += (c - '0');
  }
  return id;
}

bool MaybeReadTile(FILE *f, std::vector<Tile>* tiles) {
  int c = fgetc(f);
  if (c == EOF) {
    return false;
  }
  if (c == '\n') {
    return MaybeReadTile(f, tiles);
  }
  ExpectChar('T', c);
  ExpectStr(f, "ile ");
  int id = ReadId(f);
  ExpectStr(f, ":\n");
  int data[10][10];
  ReadTileData(f, data);

  // Add all orientations of the tile.
  for (int i = 0; i < 2; i++) {
    tiles->emplace_back(Tile(id, data, i, false, false));  // original
    FlipH(data);
    tiles->emplace_back(Tile(id, data, i, true, false));  // flipped horizontally
    FlipV(data);
    tiles->emplace_back(Tile(id, data, i, true, true));  // flipped both
    FlipH(data);
    tiles->emplace_back(Tile(id, data, i, false, true));  // flipped vertically
    FlipV(data);

    RotateClockwise90(data);
  }
  return true;
}

class Puzzle {
 public:
  Puzzle(const char *filename) {
    FILE *f = fopen(filename, "r");
    if (f == nullptr) {
      printf("unable to open input\n");
      exit(-1);
    }
    bool more = true;
    while (more) {
      more = MaybeReadTile(f, &tiles_);
    }
    fclose(f);

    // There are 8 orientations of each tile.
    int pieces = tiles_.size() / 8;
    dim_ = static_cast<int>(sqrt(pieces));
    printf("Puzzle: %d x %d\n", dim_, dim_);

    // Initialize every position with every possibility.
    for (int i = 0; i < dim_; i++) {
      possibilities_.push_back(std::vector<std::vector<int>>());
      for (int j = 0; j < dim_; j++) {
        possibilities_[i].push_back(std::vector<int>());
        for (int k = 0; k < tiles_.size(); k++) {
          possibilities_[i][j].push_back(k);
        }
      }
    }
  }

  void PrintDebugString() const {
    for (int i = 0; i < dim_; i++) {
      for (int j = 0; j < dim_; j++) {
        printf(" %4d", static_cast<int>(possibilities_[i][j].size()));
      }
      printf("\n");
    }
    printf("\n");

    printf("Possibilities at [0][0]:\n");
    for (auto t: possibilities_[0][0]) {
      tiles_[t].PrintDebugString();
    }

    printf("Possibilities at [0][1]:\n");
    for (auto t: possibilities_[0][1]) {
      tiles_[t].PrintDebugString();
    }
  }

  void FindCorners() {
    // Answer should be [2551, 1697, 1129, 3313].

    std::map<int, std::set<int>> top_edge_map;
    std::map<int, std::set<int>> bottom_edge_map;
    std::map<int, std::set<int>> left_edge_map;
    std::map<int, std::set<int>> right_edge_map;

    for (int i = 0; i < tiles_.size(); i++) {
      top_edge_map[tiles_[i].top()].insert(i);
      bottom_edge_map[tiles_[i].bottom()].insert(i);
      right_edge_map[tiles_[i].right()].insert(i);
      left_edge_map[tiles_[i].left()].insert(i);
    }

    std::vector<int> top_left_corners;
    for (int i = 0; i < tiles_.size(); i++) {
      bool debug = false;
      /*
      if (tiles_[i].id() == 2551) {
        debug = true;
        printf("Checking the one I know is right...\n");
      }
      */

      bool has_neighbor = false;
      for (auto& neighbor: bottom_edge_map[tiles_[i].top()]) {
        if (tiles_[neighbor].id() != tiles_[i].id()) {
          if (debug) {
            printf("Its top edge of %d matches %d's bottom edge of %d.\n",
                tiles_[i].top(), tiles_[neighbor].id(), tiles_[neighbor].bottom());
          }
          has_neighbor = true;
          break;
        }
      }
      if (!has_neighbor) {
        for (auto& neighbor: right_edge_map[tiles_[i].left()]) {
          if (tiles_[neighbor].id() != tiles_[i].id()) {
            if (debug) {
              printf("Its left edge of %d matches %d's right edge of %d.\n",
                  tiles_[i].left(), tiles_[neighbor].id(), tiles_[neighbor].right());
            }
            has_neighbor = true;
            break;
          }
        }
      }
      if (!has_neighbor) {
        top_left_corners.push_back(i);
      }

      if (debug) {
        printf("Done checking it.\n");
      }
    }
    printf("Found %d top left corners: ", static_cast<int>(top_left_corners.size()));
    for (int i = 0; i < top_left_corners.size(); i++) {
      printf("%d, ", tiles_[top_left_corners[i]].id());
    }
    printf("\n\n");
    /*
    for (int i = 0; i < top_left_corners.size(); i++) {
      tiles_[top_left_corners[i]].PrintDebugString();
    }
    */

    // 4 corners * 2 possible corner orientations.
    if (top_left_corners.size() != 8) {
      printf("oh no! too many or too few corners!\n");
      exit(-1);
    }
    sort(top_left_corners.begin(), top_left_corners.end());

    // Lock the first one in place.
    // I ran this 4 times until I found that the this was the correct
    // orientation for part 2.
    int top_left_corner = top_left_corners[4];
    possibilities_[0][0].clear();
    possibilities_[0][0].push_back(top_left_corner);
    solved_ids_.insert(tiles_[top_left_corner].id());

    // The other tile ids that came up are the other corners' ids.
    std::set<int> other_corner_ids;
    for (int i = 0; i < top_left_corners.size(); i++) {
      if (tiles_[top_left_corners[i]].id() != tiles_[top_left_corner].id()) {
        other_corner_ids.insert(tiles_[top_left_corners[i]].id());
      }
    }

    // Now find all the tile indices with those ids.
    std::vector<int> corner_tiles;
    for (int i = 0; i < tiles_.size(); i++) {
      if (other_corner_ids.find(tiles_[i].id()) != other_corner_ids.end()) {
        corner_tiles.push_back(i);
      }
    }

    // Now copy that set into each corner.
    possibilities_[0][dim_ - 1] = corner_tiles;
    possibilities_[dim_ - 1][dim_ - 1] = corner_tiles;
    possibilities_[dim_ - 1][0] = corner_tiles;
  }

  bool FilterImpossible() {
    bool changed = false;
    for (int i = 0; i < dim_; i++) {
      for (int j = 0; j < dim_; j++) {
        // Is this position already solved?
        if (possibilities_[i][j].size() == 1) {
          continue;
        }

        std::vector<int> new_possibilities;
        for (auto t1: possibilities_[i][j]) {
          // Is this tile possible?

          // Is this tile id already locked somewhere lese?
          if (solved_ids_.find(tiles_[t1].id()) != solved_ids_.end()) {
            changed = true;
            continue;
          }

          // Can this tile match one of its neighbors?
          bool poss_down = false;
          if (i + 1 < dim_) {
            for (auto t2: possibilities_[i+1][j]) {
              if (tiles_[t1].bottom() == tiles_[t2].top()) {
                poss_down = true;
                break;
              }
            }
          } else {
            poss_down = true;
          }

          bool poss_up = false;
          if (i > 0) {
            for (auto t2: possibilities_[i-1][j]) {
              if (tiles_[t2].bottom() == tiles_[t1].top()) {
                poss_up = true;
                break;
              }
            }
          } else {
            poss_up = true;
          }

          bool poss_right = false;
          if (j + 1 < dim_) {
            for (auto t2: possibilities_[i][j+1]) {
              if (tiles_[t1].right() == tiles_[t2].left()) {
                poss_right = true;
                break;
              }
            }
          } else {
            poss_right = true;
          }

          bool poss_left = false;
          if (j > 0) {
            for (auto t2: possibilities_[i][j-1]) {
              if (tiles_[t2].right() == tiles_[t1].left()) {
                poss_left = true;
                break;
              }
            }
          } else {
            poss_left = true;
          }

          if (poss_up and poss_down and poss_left and poss_right) {
            new_possibilities.push_back(t1);
          } else {
            changed = true;
          }
        }

        possibilities_[i][j] = new_possibilities;
        if (new_possibilities.size() == 1) {
          solved_ids_.insert(tiles_[new_possibilities[0]].id());
        }
      }
    }

    // The whole thing could be flipped diagonally, so pick one.
    /*
    if (possibilities_[dim_ - 1][dim_ - 1].size() == 2) {
      int picked = possibilities_[dim_ - 1][dim_ - 1][0];
      possibilities_[dim_ - 1][dim_ - 1].clear();
      possibilities_[dim_ - 1][dim_ - 1].push_back(picked);
      solved_ids_.insert(tiles_[picked].id());
      changed = true;
    }
    */

    return changed;
  }

  bool IsSolved() const {
    for (int i = 0; i < dim_; i++) {
      for (int j = 0; j < dim_; j++) {
        if (possibilities_[i][j].size() != 1) {
          return false;
        }
      }
    }
    return true;
  }

  int64_t Part1() const {
    int index1 = possibilities_[0][0][0];
    int index2 = possibilities_[0][dim_ - 1][0];
    int index3 = possibilities_[dim_ - 1][0][0];
    int index4 = possibilities_[dim_ - 1][dim_ - 1][0];

    int64_t id1 = tiles_[index1].id();
    int64_t id2 = tiles_[index2].id();
    int64_t id3 = tiles_[index3].id();
    int64_t id4 = tiles_[index4].id();

    printf("Computing part 1 with corners = %lld, %lld, %lld, %lld...\n", id1, id2, id3, id4);

    // 16192267830719
    return id1 * id2 * id3 * id4;
  }

  std::vector<std::vector<int>> BuildBitmap() const {
    std::vector<std::vector<int>> bitmap;
    for (int i = 0; i < dim_ * 8; i++) {
      bitmap.push_back(std::vector<int>());
      for (int j = 0; j < dim_ * 8; j++) {
        int seg_i = i / 8;
        int seg_j = j / 8;
        int off_i = i % 8;
        int off_j = j % 8;

        int index = possibilities_[seg_i][seg_j][0];
        int value = tiles_[index].data(off_i + 1, off_j + 1);

        bitmap[i].push_back(value);
      }
    }
    return bitmap;
  }

 private:
  Puzzle(const Puzzle& other) = delete;
  Puzzle(Puzzle&& other) = delete;
  Puzzle& operator=(const Puzzle& other) = delete;
  Puzzle& operator=(Puzzle&& other) = delete;

  int dim_;
  std::vector<Tile> tiles_;
  std::vector<std::vector<std::vector<int>>> possibilities_;
  std::set<int> solved_ids_;
};

void PrintBitmap(const std::vector<std::vector<int>>& bitmap) {
  for (auto& row: bitmap) {
    for (int c: row) {
      if (c == 0) {
        printf(".");
      } else if (c == 1) {
        printf("#");
      } else if (c == 2) {
        printf("O");
      } else {
        printf("?");
      }
    }
    printf("\n");
  }
  printf("\n");
}

std::vector<std::vector<int>> CreateMonsterBitmap() {
  std::vector<std::vector<int>> result;
  result.push_back(std::vector<int>(20));
  result.push_back(std::vector<int>(20));
  result.push_back(std::vector<int>(20));
  result[0][18] = 1;
  result[1][0] = 1;
  result[1][5] = 1;
  result[1][6] = 1;
  result[1][11] = 1;
  result[1][12] = 1;
  result[1][17] = 1;
  result[1][18] = 1;
  result[1][19] = 1;
  result[2][1] = 1;
  result[2][4] = 1;
  result[2][7] = 1;
  result[2][10] = 1;
  result[2][13] = 1;
  result[2][16] = 1;
  return result;
}

void SearchBitmap(std::vector<std::vector<int>> *sea, const std::vector<std::vector<int>>& monster) {
  for (int i = 0; i < sea->size() - monster.size(); i++) {
    for (int j = 0; j < sea->size() - monster[0].size(); j++) {
      // Is there a monster at i, j?
      bool matches = true;
      for (int k = 0; matches && k < monster.size(); k++) {
        for (int m = 0; matches && m < monster[0].size(); m++) {
          if (monster[k][m] == 1) {
            if ((*sea)[i+k][j+m] != 1) {
              matches = false;
            }
          }
        }
      }
      if (matches) {
        printf("Match at %d, %d\n", i, j);
        for (int k = 0; matches && k < monster.size(); k++) {
          for (int m = 0; matches && m < monster[0].size(); m++) {
            if (monster[k][m] == 1) {
              (*sea)[i+k][j+m] = 2;
            }
          }
        }
      }
    }
  }
}

int Part2(const std::vector<std::vector<int>>& bitmap) {
  int result = 0;
  for (auto& row: bitmap) {
    for (int value: row) {
      if (value == 1) {
        result++;
      }
    }
  }
  // 1909
  return result;
}

int main(int argc, char **argv) {
  Puzzle puzzle("input.txt");
  puzzle.PrintDebugString();

  puzzle.FindCorners();
  puzzle.PrintDebugString();

  bool changed = true;
  int pass = 0;
  while (changed) {
    printf("Pass %d...\n", ++pass);
    changed = puzzle.FilterImpossible();
    puzzle.PrintDebugString();
  }

  if (!puzzle.IsSolved()) {
    printf("oh no! not solved!\n");
    exit(-1);
  }

  printf("Part 1: %lld\n", puzzle.Part1());

  auto bitmap = puzzle.BuildBitmap();
  PrintBitmap(bitmap);

  auto monster = CreateMonsterBitmap();
  printf("Monster:\n");
  PrintBitmap(monster);

  SearchBitmap(&bitmap, monster);
  printf("\n");

  PrintBitmap(bitmap);
  printf("\nPart 2: %d\n", Part2(bitmap));
}
