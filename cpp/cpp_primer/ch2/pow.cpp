#include <iostream>

int main()
{
    int result = 1, pow = 10, value = 2;
    for (int i = 1; i <= pow; i++) {
        result *= value;
    }
    std::cout << "2 pow 10 is " << result << std::endl;
}