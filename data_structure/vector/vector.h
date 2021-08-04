#include <cstdio>
#include <cstdlib>

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
    void bubbleSort(Rank lo, Rank hi);
    void mergeSort(Rank lo, Rank hi);
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

    Rank size() const { return _size; }
    // TODO: 容量打印结果不对
    int capacity() const { return _capacity; }

    T& operator[] (Rank r);
    const T& operator[] (Rank r) const;

    T remove(Rank r);
    int remove(Rank lo, Rank hi);
    Rank insert(Rank r, T const& e);
    Rank insert(T const& e) { return insert(_size, e); }
    void unsort(Rank lo, Rank hi);
    void unsort() { unsort(0, _size); }
    void sort(Rank lo, Rank hi);
    void sort() { sort(0, _size); }

    void traverse( void ( * ) ( T& ) );
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
void p(T& x) {
    printf("%d,", x);
}

template <typename T>
void print(Vector<T> v) {
    printf("size:%d, cap:%d\n", v.size(), v.capacity());
    v.traverse(p);
    printf("\n");
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
void Vector<T>::shrink() {
    if(_capacity >> 1 < DEFAULT_CAPACITY) return;
    if(_size > _capacity >> 2) return;
    T* oldElem = _elem;
    _elem = new T[_capacity >>= 2];
    for(int i=0; i < _size; i++) {
       _elem[i] = oldElem[i]; 
    }
    delete [] oldElem;
}

template <typename T>
int Vector<T>::remove(Rank lo, Rank hi) {
    if(lo == hi) return 0;
    while(hi < _size) _elem[lo++] = _elem[hi++];
    _size = lo;
    shrink();
    return hi - lo;
}

template <typename T>
T Vector<T>::remove(Rank r) {
    T t = _elem[r];
    remove(r, r + 1);
    return t;
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

template <typename T>
void Vector<T>::traverse(void (*visit)( T& )) {
    for(int i=0; i < _size; i++) {
        visit(_elem[i]);
    }
}

template <typename T>
void Vector<T>::unsort(Rank lo, Rank hi) {
    T* V = _elem + lo;
    for(Rank i = hi - lo; i > 0; i--) {
        T t = V[i-1];
        int r = rand() % i;
        V[i-1] = V[r];
        V[r] = t;
    }
}

template <typename T>
void Vector<T>::bubbleSort(Rank lo, Rank hi) {
        
}
