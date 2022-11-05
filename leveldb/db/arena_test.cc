#include "arena.h"
#include "random.h"
#include "gtest/gtest.h"

namespace leveldb {
TEST(ArenaTest, Empty) { Arena arena; }

TEST(ArenaTest, Simple) {
  std::vector<std::pair<size_t, char *>> allocated;
  Arena arena;
  const int N = 100000;
}
} // namespace leveldb
