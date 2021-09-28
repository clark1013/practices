#include <iostream>

using namespace std;

// 结构体
struct VectorV1 {
  int sz;
  double *elem;
};

void vector_init(VectorV1 &v, int s) {
  v.elem = new double[s];
  v.sz = s;
}

double read_and_sum_v1(int s) {
  VectorV1 v;
  vector_init(v, s);
  for (int i = 0; i < s; i++)
    v.elem[i] = i;

  double sum = 0;
  for (int i = 0; i < s; i++)
    sum += v.elem[i];
  return sum;
}

// 类
class VectorV2 {
private:
  int sz;
  double *elem;

public:
  VectorV2(int s) : elem{new double[s]}, sz{s} {}
  double &operator[](int i) { return elem[i]; }
  int size() { return sz; }
};

double read_and_sum_v2(int s) {
  VectorV2 v(10);
  for (int i = 0; i < s; ++i)
    v[i] = i;

  double sum = 0;
  for (int i = 0; i != v.size(); ++i)
    sum += v[i];
  return sum;
}

int main(int argc, char **argv) {
  cout << read_and_sum_v1(10) << "\n";
  cout << read_and_sum_v2(10) << "\n";
}
