
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <optional>
#include <vector>

class Tile {
 public:
  Tile(int id, int data[10][10]) {
    id_ = id;
    for (int i = 0; i < 10; i++) {
      for (int j = 0; j < 10; j++) {
        data_.push_back(data[i][j]);
      }
    }

    top_ = bottom_ = right_ = left_ = 0;
    top_flip_ = bottom_flip_ = right_flip_ = left_flip_ = 0;

    for (int i = 0; i < 10; i++) {
      top_ <<= 1;
      top_ |= data_[i];

      bottom_ <<= 1;
      bottom_ |= data_[90 + i];

      left_ <<= 1;
      left_ |= data_[10 * i];

      right_ <<= 1;
      right_ |= data_[10 * i + 9];

      top_flip_ <<= 1;
      top_flip_ |= data_[9 - i];

      bottom_flip_ <<= 1;
      bottom_flip_ |= data_[99 - i];

      left_flip_ <<= 1;
      left_flip_ |= data_[10 * (9 - i)];

      right_flip_ <<= 1;
      right_flip_ |= data_[10 * (9 - i) + 9];
    }
  }

  Tile(Tile &&other) {
    id_ = other.id_;
    data_ = std::move(other.data_);
    other.id_ = 0;
  }

  Tile &operator=(Tile &&other) {
    id_ = other.id_;
    data_ = std::move(other.data_);
    other.id_ = 0;
    return *this;
  }

  int top() { return top_; }
  int bottom() { return bottom_; }
  int left() { return left_; }
  int right() { return right_; }

  void PrintDebugString() {
    printf("Tile %4d:\n", id_);
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
  int top_flip_, bottom_flip_, left_flip_, right_flip_;
};

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

std::optional<Tile> MaybeReadTile(FILE *f) {
  int c = fgetc(f);
  if (c == EOF) {
    return std::nullopt;
  }
  if (c == '\n') {
    return MaybeReadTile(f);
  }
  ExpectChar('T', c);
  ExpectStr(f, "ile ");
  int id = ReadId(f);
  ExpectStr(f, ":\n");
  int data[10][10];
  ReadTileData(f, data);

  return std::make_optional(Tile(id, data));
}

int main(int argc, char **argv) {
  FILE *f = fopen("input.txt", "r");
  if (f == nullptr) {
    printf("unable to open input\n");
    exit(-1);
  }

  std::optional<Tile> tile = std::move(MaybeReadTile(f));
  while (tile) {
    tile->PrintDebugString();
    tile = MaybeReadTile(f);
  }

  fclose(f);
}
