#include <algorithm>
#include <fstream>
#include <iostream>

template <class T> class Array2D {
public:
  Array2D() : array_(0){};
  ~Array2D() {
    delete[] array_;
    array_ = 0;
  }
  void setSize(int width, int height) {
    width_ = width;
    height_ = height;
    array_ = new T[width * height];
  }
  T &operator()(int x, int y) { return array_[width_ * y + x]; }
  const T &operator()(int x, int y) const { return array_[width_ * y + x]; }

private:
  T *array_;
  int width_;
  int height_;
};

class State {
public:
  State(const char *stageData, int size);
  void draw() const;
  void update(char input);
  bool win() const;

private:
  enum Object {
    OBJ_SPACE,
    OBJ_WALL,
    OBJ_BLOCK,
    OBJ_MAN,
    OBJ_UNKNOWN,
  };
  void setSize(const char *stageData, int size);

  int height_;
  int width_;
  Array2D<Object> objects_;
  Array2D<bool> goalFlags_;
};

State::State(const char *stageData, int size) {
  setSize(stageData, size);
  objects_.setSize(width_, height_);
  goalFlags_.setSize(width_, height_);
  for (int y = 0; y < height_; y++) {
    for (int x = 0; x < width_; x++) {
      objects_(x, y) = OBJ_WALL;
      goalFlags_(x, y) = false;
    }
  }

  int x = 0;
  int y = 0;
  for (int i = 0; i < size; i++) {
    Object t;
    bool flag = false;
    switch (stageData[i]) {
    case ' ':
      t = OBJ_SPACE;
      break;
    case '#':
      t = OBJ_WALL;
      break;
    case '.':
      t = OBJ_SPACE;
      flag = true;
      break;
    case 'o':
      t = OBJ_BLOCK;
      break;
    case 'O':
      t = OBJ_BLOCK;
      flag = true;
      break;
    case 'p':
      t = OBJ_MAN;
      break;
    case 'P':
      t = OBJ_MAN;
      flag = true;
      break;
    case '\n':
      x = 0;
      y++;
      t = OBJ_UNKNOWN;
      break;
    default:
      t = OBJ_UNKNOWN;
      break;
    }
    if (t != OBJ_UNKNOWN) {
      // std::cout << x << "," << y << " " << t << std::endl;
      objects_(x, y) = t;
      goalFlags_(x, y) = flag;
      x++;
    }
  }
}

void State::setSize(const char *stageData, int size) {
  width_ = height_ = 0;
  int x = 0;
  int y = 0;
  for (int i = 0; i < size; i++) {
    switch (stageData[i]) {
    case ' ':
    case '#':
    case '.':
    case 'o':
    case 'O':
    case 'p':
    case 'P':
      x++;
      break;
    case '\n':
      y++;
      width_ = std::max(width_, x);
      height_ = std::max(height_, y);
      x = 0;
      break;
    }
  }
}

void State::draw() const {
  for (int y = 0; y < height_; ++y) {
    for (int x = 0; x < width_; ++x) {
      bool flag = goalFlags_(x, y);
      Object obj = objects_(x, y);
      switch (obj) {
      case OBJ_WALL:
        std::cout << '#';
        break;
      case OBJ_SPACE:
        if (flag) {
          std::cout << '.';
        } else {
          std::cout << ' ';
        }
        break;
      case OBJ_BLOCK:
        if (flag) {
          std::cout << 'O';
        } else {
          std::cout << 'o';
        }
        break;
      case OBJ_MAN:
        if (flag) {
          std::cout << 'P';
        } else {
          std::cout << 'p';
        }
        break;
      default:
        break;
      }
    }
    std::cout << '\n';
  }
  std::cout << std::endl;
}

void State::update(char input) {
  int dx = 0;
  int dy = 0;
  switch (input) {
  case 'w':
    dy = -1;
    break;
  case 'a':
    dx = -1;
    break;
  case 's':
    dy = 1;
    break;
  case 'd':
    dx = 1;
    break;
  }

  // people's position
  int x;
  int y;
  bool found = false;
  for (y = 0; y < height_; y++) {
    for (x = 0; x < width_; x++) {
      if (objects_(x, y) == OBJ_MAN) {
        found = true;
        break;
      }
    }
    if (found) {
      break;
    }
  }

  // target position
  int tx = x + dx;
  int ty = y + dy;
  if (tx < 0 || ty < 0 || tx >= width_ || ty >= height_) {
    return;
  }
  std::cout << x << "," << y << "," << tx << "," << ty << std::endl;

  // people/target index
  Object target = objects_(tx, ty);
  // if target is space or goal, just move to the target
  if (target == OBJ_SPACE) {
    objects_(tx, ty) = OBJ_MAN;
    objects_(x, y) = OBJ_SPACE;
  } else if (target == OBJ_BLOCK) {
    // target next position
    int ttx = tx + dx;
    int tty = ty + dy;
    if (objects_(ttx, tty) == OBJ_SPACE) {
      objects_(ttx, tty) = OBJ_BLOCK;
      objects_(tx, ty) = OBJ_MAN;
      objects_(x, y) = OBJ_SPACE;
    }
  }
}

bool State::win() const {
  for (int y = 0; y < height_; y++) {
    for (int x = 0; x < width_; x++) {
      bool flag = goalFlags_(x, y);
      Object obj = objects_(x, y);
      if (flag && obj != OBJ_BLOCK) {
        return false;
      }
    }
  }
  return true;
}

void readFile(char **buffer, int *size, const char *filename) {
  std::ifstream in(filename);
  if (!in) {
    *buffer = 0;
    *size = 0;
  } else {
    in.seekg(0, std::ifstream::end);
    *size = static_cast<int>(in.tellg());
    in.seekg(0, std::ifstream::beg);
    *buffer = new char[*size];
    in.read(*buffer, *size);
  }
}

const char gStageData[] = "\
########\n\
# .. p #\n\
# oo   #\n\
#      #\n\
########\n";
const int gStageWidth = 8;
const int gStageHeight = 5;

int main(int argc, char **argv) {
  const char *filename = "stageData.txt";
  if (argc >= 2) {
    filename = argv[1];
  }
  char *stageData;
  int fileSize;
  readFile(&stageData, &fileSize, filename);
  State *state = new State(stageData, fileSize);
  while (true) {
    state->draw();
    std::cout << "a:left d:right w:up s:down. command?" << std::endl;
    char input;
    std::cin >> input;
    state->update(input);
    if (state->win()) {
      break;
    }
  }
  std::cout << "Congrats, You won the game!" << std::endl;
  delete state;
}
