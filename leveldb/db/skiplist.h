#include "arena.h"
#include "random.h"

namespace leveldb {

template <typename Key, class Comparator> class SkipList {
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

template <typename Key, class Comparator>
struct SkipList<Key, Comparator>::Node {
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

template <typename Key, class Comparator>
typename SkipList<Key, Comparator>::Node *
SkipList<Key, Comparator>::NewNode(const Key &key, int height) {}
} // namespace leveldb
