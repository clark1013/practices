#include <iostream>

using namespace std;

void hello() { std::cout << "Hello World!\n"; }

double square(double x) { return x * x; }

void print_square(double x) {
  cout << "the square of " << x << " is " << square(x) << "\n";
}

void copy_fct() {
  int v1[10] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
  int v2[10];

  for (int i = 0; i < 10; i++) {
    v2[i] = v1[i];
  }

  for (auto &x : v2)
    ++x;

  for (auto x : v2)
    cout << x << "\n";
}

int main(int argc, char **argv) {
  hello();
  print_square(1.34);
  copy_fct();
}
