#include <iostream>
#include <memory>

class Empty {
public:
  Empty() : a(1), b(2) { std::cout << "Init" << std::endl; }
  Empty(const Empty &) { std::cout << "Copy" << std::endl; }
  // Empty(const Empty &) = delete;
  const Empty &operator=(const Empty &t) {
    std::cout << "Assign" << std::endl;
    return t;
  }
  // Empty &operator=(const Empty &) = delete;
  ~Empty() { std::cout << "Destroy" << std::endl; }

private:
  int a, b;
};

int main() {
  auto e = std::make_shared<Empty>();
  std::cout << "1" << std::endl;
  auto e1(e);
  std::cout << "2" << std::endl;
  auto e2 = std::make_shared<Empty>();
  std::cout << "3" << std::endl;

  e2 = e;
  std::cout << "4" << std::endl;
}
