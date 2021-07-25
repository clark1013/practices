#include <cstdio>

typedef int Rank;
#define DEFAULT_CAPACITY 3

template <typename T> class Vector {
protected:
    Rank _size;
    int _capacity;
    T* _elem;
    void copyFrom(T const* A, Rank lo, Rank hi);  // 复制数组区间A[lo, hi)
    void expand();  // 扩容
    void shrink();  // 缩容
public:
    // 构造函数
    Vector(int c = DEFAULT_CAPACITY, int s = 0, T v = 0) {
	_elem = new T[_capacity=c];
	for (_size = 0; _size < s; _size++) {
	    _elem[_size] = v;
	}
    }
    Vector(T const* A, Rank lo, Rank hi) { copyFrom(A, lo, hi); }
    Vector(T const* A, Rank n) { copyFrom(A, 0, n); }
    Vector(Vector<T> const& V, Rank lo, Rank hi) { copyFrom(V._elem, lo, hi); }
    Vector(Vector<T> const& V) { copyFrom(V._elem, 0, V._size); }
    // 析构函数
    ~Vector() { delete [] _elem; }

    Rank size() { return _size; }
    int capacity() { return _capacity; }

    T& operator[] (Rank r);
    const T& operator[] (Rank r) const;

    Rank insert(Rank r, T const& e);
    Rank insert(T const& e) { return insert(_size, e); }
};


// 重载下标操作符
template <typename T>
T& Vector<T>::operator[] (Rank r) {
    return _elem[r];
}

template <typename T>
const T& Vector<T>::operator[] (Rank r) const {
    return _elem[r];
}

template <typename T>
void print(Vector<T> v) {
    printf("size:%d, cap:%d\n", v.size(), v.capacity());
}

template <typename T>
void Vector<T>::copyFrom(T const* A, Rank lo, Rank hi) {
    _elem = new T[(hi - lo) * 2];
    _size = 0;
    while (lo < hi) {
	_elem[_size++] = A[lo++];
    }
}

template <typename T>
void Vector<T>::expand() {
    if(_size < _capacity) return;
    if(_capacity < DEFAULT_CAPACITY) _capacity = DEFAULT_CAPACITY;
    T* oldElem = _elem;
    _elem = new T[ _capacity <<= 1 ];
    for(int i=0; i < _size; i++) {
	_elem[i] = oldElem[i];
    }
    delete [] oldElem;
}

template <typename T>
Rank Vector<T>::insert(Rank r, T const& e) {
    expand();
    for (int i=_size; i > r; i--) {
	_elem[i] = _elem[i-1];
    }
    _elem[r] = e;
    _size++;
    return r;
}
