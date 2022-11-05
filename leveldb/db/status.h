#include "slice.h"

namespace leveldb {
class Status {
public:
  Status() noexcept : state_(nullptr) {}
  ~Status() { delete[] state_; }
  Status(const Status &rhs);
  Status &operator=(const Status &rhs);

  // Return a success status.
  static Status OK() { return Status(); }

  // Return error status of an appropriate type
  static Status NotFound(const Slice &msg, const Slice &msg2 = Slice()) {
    return Status(kNotFound, msg, msg2);
  }
  static Status Corruption(const Slice &msg, const Slice &msg2 = Slice()) {
    return Status(kCorruption, msg, msg2);
  }
  static Status NotSupported(const Slice &msg, const Slice &msg2 = Slice()) {
    return Status(kNotSupported, msg, msg2);
  }
  static Status InvalidArgument(const Slice &msg, const Slice &msg2 = Slice()) {
    return Status(kInvalidArgument, msg, msg2);
  }
  static Status IOError(const Slice &msg, const Slice &msg2 = Slice()) {
    return Status(kIOError, msg, msg2);
  }

  // Return true if the code match
  bool ok() const { return state_ == nullptr; }
  bool IsNotFound() const { return code() == kNotFound; }
  bool IsCorruption() const { return code() == kCorruption; }
  bool IsNotSupported() const { return code() == kNotSupported; }
  bool IsInvalidArgument() const { return code() == kInvalidArgument; }
  bool IsIOError() const { return code() == kIOError; }

private:
  enum Code {
    kOk = 0,
    kNotFound = 1,
    kCorruption = 2,
    kNotSupported = 3,
    kInvalidArgument = 4,
    kIOError = 5,
  };

  Code code() const {
    return (state_ == nullptr) ? kOk : static_cast<Code>(state_[4]);
  }

  Status(Code code, const Slice &msg, const Slice &msg2);

  static const char *CopyState(const char *s);
  // OK status has a nullptr state_. Otherwise, state_ is a new[] array.
  // of the following form:
  //   state[0..3] = message length
  //   state[4] = code
  //   state[5..] = message
  const char *state_;
};

inline Status::Status(const Status &rhs) {
  state_ = (rhs.state_ == nullptr) ? nullptr : CopyState(rhs.state_);
}

inline Status &Status::operator=(const Status &rhs) {
  if (state_ != rhs.state_) {
    delete[] state_;
    state_ = (rhs.state_ == nullptr) ? nullptr : CopyState(rhs.state_);
  }
  return *this;
}
} // namespace leveldb
