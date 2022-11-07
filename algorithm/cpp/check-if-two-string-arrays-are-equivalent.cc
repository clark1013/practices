#include <iostream>
#include <string>
#include <vector>

using std::string;
using std::vector;

class Solution {
public:
  bool arrayStringsAreEqual(vector<string> &word1, vector<string> &word2) {
    vector<char> temp;
    for (auto s = word1.begin(); s != word1.end(); ++s) {
      for (auto c = (*s).begin(); c != (*s).end(); ++c) {
        temp.push_back((*c));
      }
    }
    size_t size1 = temp.size();
    size_t t = 0;
    for (auto s = word2.begin(); s != word2.end(); ++s) {
      for (auto c = (*s).begin(); c != (*s).end(); ++c) {
        if (t > size1 - 1) {
          return false;
        }
        if ((*c) != temp[t]) {
          return false;
        }
        t++;
      }
    }
    if (size1 != t) {
      return false;
    }
    return true;
  }
};

int main() {
  Solution solution;
  vector<string> w1;
  w1.push_back("ab");
  w1.push_back("c");
  vector<string> w2;
  w2.push_back("a");
  w2.push_back("bc");
  bool result = solution.arrayStringsAreEqual(w1, w2);
  std::cout << result << std::endl;
  return 0;
}
