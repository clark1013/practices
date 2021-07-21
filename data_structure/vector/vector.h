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
};
