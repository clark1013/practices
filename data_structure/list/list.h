#include <cstddef>

typedef int Rank;

template <typename T> struct ListNode {
    T data;
    ListNode<T>* pred;
    ListNode<T>* succ;

    ListNode() {}  // 针对 header 和 tailer 的构造
    ListNode(T e, ListNode<T>* p = NULL, ListNode<T>* s = NULL) 
        : data(e), pred(p), succ(s) {}

    ListNode<T>* insertAsPred(T const& e);
    ListNode<T>* insertasSucc(T const& e);
};

template <typename T>
ListNode<T>* ListNode<T>::insertAsPred(T const& e) {
    ListNode<T>* n = new ListNode(e, pred, this);
    pred->succ = n;
    pred = n;
    return n;
}

template <typename T>
ListNode<T>* ListNode<T>::insertAsSucc(T const& e) {
    ListNode<T>* n = new ListNode(e, this, succ);
    succ->pred = n;
    succ = n;
    return n;
}
