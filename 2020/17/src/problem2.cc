
#include <cstdio>
#include <cstdlib>
#include <map>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

using std::string;
using std::vector;

class Space {
 public:
  Space(): minx_(0), maxx_(0),
           miny_(0), maxy_(0),
           minz_(0), maxz_(0),
           minw_(0), maxw_(0),
           count_(0) {}

  Space(const Space& other) = delete;

  Space(Space&& other): minx_(0), maxx_(0),
                        miny_(0), maxy_(0),
                        minz_(0), maxz_(0),
                        minw_(0), maxw_(0),
                        count_(0) {
    count_ = other.count_;
    minx_ = other.minx_;
    miny_ = other.miny_;
    minz_ = other.minz_;
    minw_ = other.minw_;
    maxx_ = other.maxx_;
    maxy_ = other.maxy_;
    maxz_ = other.maxz_;
    maxw_ = other.maxw_;
    data_ = std::move(other.data_);
  }

  Space& operator=(const Space& other) = delete;

  Space& operator=(Space&& other) {
    if (this == &other) {
      return *this;
    }
    count_ = other.count_;
    minx_ = other.minx_;
    miny_ = other.miny_;
    minz_ = other.minz_;
    minw_ = other.minw_;
    maxx_ = other.maxx_;
    maxy_ = other.maxy_;
    maxz_ = other.maxz_;
    maxw_ = other.maxw_;
    data_ = std::move(other.data_);
    return *this;
  }

  void Set(int x, int y, int z, int w) {
    if (x < minx_) minx_ = x;
    if (y < miny_) miny_ = y;
    if (z < minz_) minz_ = z;
    if (w < minw_) minw_ = w;
    if (x > maxx_) maxx_ = x;
    if (y > maxy_) maxy_ = y;
    if (z > maxz_) maxz_ = z;
    if (w > maxw_) maxw_ = w;
    data_[std::make_tuple(x, y, z, w)] = true;
    count_++;
  }

  bool Get(int x, int y, int z, int w) const {
    auto it = data_.find(std::make_tuple(x, y, z, w));
    if (it == data_.end()) {
      return false;
    }
    return true;
  }

  int minx() const { return minx_; }
  int miny() const { return miny_; }
  int minz() const { return minz_; }
  int minw() const { return minw_; }
  int maxx() const { return maxx_; }
  int maxy() const { return maxy_; }
  int maxz() const { return maxz_; }
  int maxw() const { return maxw_; }

  int count() const { return count_; }

 private:
  int count_;
  int minx_, miny_, minz_, minw_, maxx_, maxy_, maxz_, maxw_;
  std::map<std::tuple<int, int, int, int>, bool> data_;
};

Space ReadLayer(const char *path) {
  Space space;
  FILE *f = fopen(path, "r");
  if (f == nullptr) {
    printf("unable to open file");
    exit(-1);
  }
  int x = 0;
  int y = 0;
  int c;
  while ((c = fgetc(f)) != EOF) {
    if (c == '\n') {
      x = 0;
      y++;
    } else {
      if (c == '#') {
        space.Set(x, y, 0, 0);
      }
      x++;
    }
  }
  return space;
}

void PrintSpace(const Space &space) {
  for (int w = space.minw(); w <= space.maxw(); w++) {
    for (int z = space.minz(); z <= space.maxz(); z++) {
      printf("z = %d, w = %d\n", z, w);
      for (int y = space.miny(); y <= space.maxy(); y++) {
        for (int x = space.minx(); x <= space.maxx(); x++) {
          if (space.Get(x, y, z, w)) {
            printf("#");
          } else {
            printf(".");
          }
        }
        printf("\n");
      }
      printf("\n");
    }
  }
}

int CountNeighbors(const Space& input, int x, int y, int z, int w) {
  int count = 0;
  for (int dx = -1; dx <= 1; dx++) {
    for (int dy = -1; dy <= 1; dy++) {
      for (int dz = -1; dz <= 1; dz++) {
        for (int dw = -1; dw <= 1; dw++) {
          if (dx != 0 || dy != 0 || dz != 0 || dw != 0) {
            if (input.Get(x + dx, y + dy, z + dz, w + dw)) {
              count++;
            }
          }          
        }
      }
    }
  }
  return count;
}

Space Process(const Space& input) {
  Space output;
  for (int w = input.minw() - 1; w <= input.maxw() + 1; w++) {
    for (int z = input.minz() - 1; z <= input.maxz() + 1; z++) {
      for (int y = input.miny() - 1; y <= input.maxy() + 1; y++) {
        for (int x = input.minx() - 1; x <= input.maxx() + 1; x++) {
          bool old = input.Get(x, y, z, w);
          int n = CountNeighbors(input, x, y, z, w);
          if (old) {
            if (n == 2 || n == 3) {
              output.Set(x, y, z, w);
            }
          } else {
            if (n == 3) {
              output.Set(x, y, z, w);
            }
          }
        }
      }
    }
  }
  return output;
}

int main(int argc, char **argv) {
  Space space = ReadLayer("input.txt");
  // PrintSpace(space);

  for (int i = 0; i < 6; i++) {
    space = Process(space);
    printf("After %d steps...\n", i + 1);
    // PrintSpace(space);
    printf("Count: %d\n\n", space.count());
  }

  return 0;
}
