#include "status.h"
#include "gtest/gtest.h"

namespace leveldb {

TEST(Status, MoveConstructor) {
  {
    Status ok = Status::OK();
    Status ok2 = std::move(ok);
    Status ok3 = ok;

    ASSERT_TRUE(ok2.ok());
  }

  {
    Status self_moved = Status::IOError("Custom IOError status message");

    Status &self_moved_reference = self_moved;
    self_moved_reference = std::move(self_moved);
  }
}

} // namespace leveldb
