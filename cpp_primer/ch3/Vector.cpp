#include "Vector.h"
#include <stdexcept>

using namespace std;

Vector::Vector(int s) : elem{new double[s]}, sz{s} {}

double &Vector::operator[](int i) {
  if (i < 0 || i >= size()) {
    throw out_of_range{"Vector operator[]"};
  }
  return elem[i];
}

int Vector::size() { return sz; }
