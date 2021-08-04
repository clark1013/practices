#include "vector.h"

int main(int argc, char* argv[]){
    Vector<int> V;

    V.insert(0);
    V.insert(1);
    V.insert(2);
    V.insert(3);
    V.insert(4);
    V.insert(5);

    print(V);

    V.unsort();
    print(V);

    V.remove(0);
    V.remove(1,3);

    print(V);
}
