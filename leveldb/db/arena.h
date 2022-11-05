#include <atomic>
#include <cstddef>
#include <vector>

namespace leveldb {
class Arena {
public:
  Arena();
  Arena(const Arena &) = delete;
  Arena &operator=(const Arena &) = delete;
  ~Arena();

  // Return a pointer to a newly allocated memory block of "bytes" bytes.
  char *Allocate(size_t bytes);

  // Allocate memory with the normal alignment guarantees provided by malloc.
  char *AllocateAligned(size_t bytes);

  // Returns an estimate of the total memory usage of the data allocated by
  // arena.
  size_t MemoryUsage() const {
    return memory_usage_.load(std::memory_order_relaxed);
  }

private:
  char *AllocateFallback(size_t bytes);
  char *AllocateNewBlock(size_t block_bytes);

  // Allocation state
  char *alloc_ptr_;
  size_t alloc_bytes_remaining_;

  // array of new[] allocated memory blocks.
  std::vector<char *> blocks_;

  // total memory usage of arena.
  std::atomic<size_t> memory_usage_;
};

inline char *Arena::Allocate(size_t bytes) {
  assert(bytes > 0);
  if (bytes <= alloc_bytes_remaining_) {
    char *result = alloc_ptr_;
    alloc_ptr_ += bytes;
    alloc_bytes_remaining_ -= bytes;
    return result;
  }
  return AllocateFallback(bytes);
}

} // namespace leveldb
