#include <iostream>
#include <string>

using std::string;

class Solution {
public:
  int maxRepeating(string sequence, string word) {
    int ls = sequence.length();
    int lw = word.length();
    int result = 0;
    for (int i = 0; i < ls; ++i) {
      if (sequence[i] == word[0]) {
        int t = 0;
        int k = 0;
        for (int j = i; j < ls; j++) {
          if (word[k % lw] == sequence[j]) {
            if (k % lw == lw - 1) {
              t += 1;
            }
            k++;
            continue;
          } else {
            break;
          }
        }
        result = t > result ? t : result;
      }
    }
    return result;
  }
};

int main() {
  Solution s;
  std::cout << s.maxRepeating("ababc", "ab") << std::endl;
  std::cout << s.maxRepeating("ababc", "ba") << std::endl;
  std::cout << s.maxRepeating("ababc", "ac") << std::endl;
  return 0;
}

// 1 5 7 凑 n
// [n]
// [1, 2, 3, 4, 1, 2, 1 ...]
// f(n) = min(1 + f(n - 1), 1 + f(n - 5), 1 + f(n - 7))
