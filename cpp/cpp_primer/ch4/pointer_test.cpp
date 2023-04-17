#include <iostream>
#include <string>

using std::string;

int main() {
  string s("hello world");
  string *sp = &s;
  std::cout << *sp << std::endl;
  *sp = "good bye";
  std::cout << *sp << std::endl;
  for (string::iterator i = sp -> begin(); i != sp -> end(); i++) {
    std::cout << *i << std::endl;
  }
  return 0;
}

