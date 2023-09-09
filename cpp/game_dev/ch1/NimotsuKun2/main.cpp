#include <iostream>

template <class T> class Array2D {
public:
  Array2D() : mArray(0){};
  ~Array2D() {
    delete[] mArray;
    mArray = 0;
  }
  void setSize(int width, int height) {
    mWidth = width;
    mHeight = height;
    mArray = new T[width * height];
  }
  T &operator()(int x, int y) { return mArray[mWidth * y + x]; }
  const T &operator()(int x, int y) const { return mArray[mWidth * y + x]; }

private:
  T *mArray;
  int mWidth;
  int mHeight;
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

  int mHeight;
  int mWidth;
  Array2D<Object> mObjects;
  Array2D<bool> mGoalFlags;
};

void State::setSize(const char *stageData, int size) {
  mWidth = mHeight = 0;
  const char *d = stageData;
  while (*d != '\0') {
  }
}

const char gStageData[] = "\
########\n\
# .. p #\n\
# oo   #\n\
#      #\n\
########";
const int gStageWidth = 8;
const int gStageHeight = 5;

enum Object {
  OBJ_SPACE,
  OBJ_WALL,
  OBJ_GOAL,
  OBJ_BLOCK,
  OBJ_BLOCK_ON_GOAL,
  OBJ_MAN,
  OBJ_MAN_ON_GOAL,
  OBJ_UNKNOWN,
};

void initialize(Object *state, int width, int height, const char *stageData);
void draw(Object *state, int width, int height);
void update(Object *state, char input, int width, int height);
bool win(Object *state, int width, int height);

int main() {
  Object *state = new Object[gStageWidth * gStageHeight];
  initialize(state, gStageWidth, gStageHeight, gStageData);
  while (true) {
    draw(state, gStageWidth, gStageHeight);
    if (win(state, gStageWidth, gStageWidth)) {
      break;
    }
    std::cout << "a:left s:right w:up z:down. command?" << std::endl;
    char input;
    std::cin >> input;
    update(state, input, gStageWidth, gStageHeight);
  }
  std::cout << "Congrats, You won the game!" << std::endl;
  delete[] state;
  state = 0;
}

void initialize(Object *state, int width, int height, const char *stageData) {
  const char *d = stageData;
  int idx = 0;
  while (*d != '\0') {
    Object t;
    switch (*d) {
    case ' ':
      t = OBJ_SPACE;
      break;
    case '#':
      t = OBJ_WALL;
      break;
    case '.':
      t = OBJ_GOAL;
      break;
    case 'o':
      t = OBJ_BLOCK;
      break;
    case 'O':
      t = OBJ_BLOCK_ON_GOAL;
      break;
    case 'p':
      t = OBJ_MAN;
      break;
    case 'P':
      t = OBJ_MAN_ON_GOAL;
      break;
    default:
      t = OBJ_UNKNOWN;
      break;
    }
    if (t != OBJ_UNKNOWN) {
      state[idx] = t;
      ++idx;
    }
    ++d;
  }
}

void draw(Object *state, int width, int height) {
  const char fonts[] = {' ', '#', '.', 'o', 'O', 'p', 'P'};
  for (int y = 0; y < height; ++y) {
    for (int x = 0; x < width; ++x) {
      std::cout << fonts[state[y * width + x]];
    }
    std::cout << std::endl;
  }
}

void update(Object *state, char input, int width, int height) {
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
  int i = -1;
  for (i = 0; i < width * height; i++) {
    if (state[i] == OBJ_MAN || state[i] == OBJ_MAN_ON_GOAL) {
      break;
    }
  }
  int x = i % width;
  int y = i / width;

  // target position
  int tx = x + dx;
  int ty = y + dy;
  if (tx < 0 || ty < 0 || tx >= width || ty >= height) {
    return;
  }
  std::cout << x << "," << y << "," << tx << "," << ty << std::endl;

  // people/target index
  int pidx = y * width + x;
  int tidx = ty * width + tx;
  // if target is space or goal, just move to the target
  if (state[tidx] == OBJ_SPACE || state[tidx] == OBJ_GOAL) {
    state[tidx] = (state[tidx] == OBJ_GOAL) ? OBJ_MAN_ON_GOAL : OBJ_MAN;
    state[pidx] = (state[pidx] == OBJ_MAN_ON_GOAL) ? OBJ_GOAL : OBJ_SPACE;
  } else if (state[tidx] == OBJ_BLOCK || state[tidx] == OBJ_BLOCK_ON_GOAL) {
    // target next position
    int ttx = tx + dx;
    int tty = ty + dy;
    int ttidx = tty * width + ttx;
    if (state[ttidx] == OBJ_SPACE || state[ttidx] == OBJ_GOAL) {
      state[ttidx] =
          (state[ttidx] == OBJ_SPACE) ? OBJ_BLOCK : OBJ_BLOCK_ON_GOAL;
      state[tidx] =
          (state[tidx] == OBJ_BLOCK_ON_GOAL) ? OBJ_MAN_ON_GOAL : OBJ_MAN;
      state[pidx] = (state[pidx] == OBJ_MAN_ON_GOAL) ? OBJ_GOAL : OBJ_SPACE;
    }
  }
}

bool win(Object *state, int width, int height) {
  for (int i = 0; i < width * height; i++) {
    if (state[i] == OBJ_BLOCK) {
      return false;
    }
  }
  return true;
}
