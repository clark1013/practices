#include "arena.h"
#include "random.h"

namespace leveldb {

template <class Key, class Comparator> class SkipList {
private:
  struct Node;

public:
  // Create a new SkipList object that will use "cmp" for comparing keys.
  // and will allocate memory using "*arena". Objects allocated in the arena
  // must remain allocated for the lifetime of the SkipList object.
  explicit SkipList(Comparator cmp, Arena *arena);

  SkipList(const SkipList &) = delete;
  SkipList &operator=(const SkipList &) = delete;

  void Insert(const Key &key);

  bool Contains(const Key &key) const;

private:
  enum { kMaxHeight = 12 };

  inline int GetMaxHeight() const {
    return max_height_.load(std::memory_order_relaxed);
  }

  Node *NewNode(const Key &key, int height);
  int RandomHeight();
  bool Equal(const Key &a, const Key &b) const { return compare_(a, b) == 0; }

  // Immutable after construction.
  Comparator const compare_;
  Arena *const arena_;

  Node *const head_;

  // Height of the entire list.
  // Modified only by Insert(). Read racily by readers, but stale values are ok.
  std::atomic<int> max_height_;

  // Read/written only by Insert().
  Random rnd_;
};

template <class Key, class Comparator> struct SkipList<Key, Comparator>::Node {
  explicit Node(const Key &k) : key(k) {}

  Key const key;

  Node *Next(int n) {
    assert(n >= 0);
    // https://en.cppreference.com/w/cpp/atomic/memory_order#Release-Acquire_ordering
    return next_[n].load(std::memory_order_acquire);
  }

  void SetNext(int n, Node *x) {
    assert(n >= 0);
    next_[n].store(std::memory_order_release);
  }

private:
  std::atomic<Node *> next_[1];
};

template <class Key, class Comparator>
class SkipList<Key, Comparator>::Node *
SkipList<Key, Comparator>::NewNode(const Key &key, int height) {
  char *const node_memory = arena_->AllocateAligned(
      sizeof(Node) + sizeof(std::atomic<Node *>) * (height - 1));
  return new (node_memory) Node(key);
}

template <class Key, class Comparator>
SkipList<Key, Comparator>::SkipList(Comparator cmp, Arena *arena)
    : compare_(cmp), arena_(arena), head_(NewNode(0, kMaxHeight)),
      max_height_(1), rnd_(Random(0xdeadbeef)) {
  for (int i = 0; i < kMaxHeight; i++) {
    head_->SetNext(i, nullptr);
  }
}

template <class Key, class Comparator>
int SkipList<Key, Comparator>::RandomHeight() {
  static unsigned const int kBranching = 4;
  int height = 1;
  while (height < kMaxHeight && rnd_.OneIn(kBranching)) {
    height++;
  }
  assert(height > 0);
  assert(height <= kMaxHeight);
  return height;
}

template <class Key, class Comparator>
void SkipList<Key, Comparator>::Insert(const Key &key) {}

} // namespace leveldb
