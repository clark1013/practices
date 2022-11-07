#include <iostream>
#include <string>
#include <vector>

using std::string;
using std::vector;

class Solution {
public:
  bool backspaceCompare(string s, string t) {
    vector<char> sv = cal(s);
    vector<char> st = cal(t);
    return sv == st;
  }

private:
  vector<char> cal(string s) {
    vector<char> result;
    for (auto c = s.begin(); c != s.end(); ++c) {
      if (*c == '#') {
        if (result.size() > 0) {
          result.pop_back();
        }
      } else {
        result.push_back(*c);
      }
    }
    return result;
  }
};

int main() {
  Solution s = Solution();
  std::cout << s.backspaceCompare("ab#c", "ad#c") << std::endl;
  std::cout << s.backspaceCompare("####", "#") << std::endl;
  std::cout << s.backspaceCompare("ab##", "a#d#") << std::endl;
  std::cout << s.backspaceCompare("ab##", "a#d#c") << std::endl;
}
