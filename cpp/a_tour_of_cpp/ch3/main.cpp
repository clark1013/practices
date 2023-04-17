#include "Vector.h"
#include <cmath>
#include <iostream>

using namespace std;

int main(int argc, char **argv) {
  Vector v = Vector(2);
  v[0] = 16;
  v[1] = 4;
  v[2] = 25;

  double sum = 0;
  for (int i = 0; i != v.size(); i++) {
    sum += sqrt(v[i]);
  }
  cout << "the sqrt result is " << sum << "\n";
}
