#include <vector>

using std::vector;

class Solution {
public:
  bool isBoomerang(vector<vector<int>> &points) {
    vector<int> p1 = points[0];
    vector<int> p2 = points[1];
    vector<int> p3 = points[2];
    return (p1[0] - p2[0]) / (p1[1] - p2[1]) ==
           (p1[0] - p3[0]) / (p1[1] - p3[1]);
  }
};
