#include <iostream>
#include <vector>
#include <string>

using std::vector;
using std::string;
using std::cout;
using std::endl;

int main()
{
	// vector<string> svector = {"aa","bb","cc","dd"};
	vector<string> svector;
	for (int i = 0; i < 10; i++) {
		svector.push_back(std::to_string(i));
	}
	for (vector<string>::iterator v = svector.begin(); v != svector.end(); v++) {
		cout << *v << endl;
	}
	vector<string>::iterator mid = svector.begin() + svector.size() / 2;
	cout << "the mid is " << *mid << endl;
	return 0;
}
