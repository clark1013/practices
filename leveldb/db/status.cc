#include "status.h"
#include <cstdint>
#include <string>

namespace leveldb {

const char *Status::CopyState(const char *state) {
  uint32_t size;
  std::memcpy(&size, state, sizeof(size));
  char *result = new char[size + 5];
  std::memcpy(result, state, size + 5);
  return result;
}

Status::Status(Code code, const Slice &msg, const Slice &msg2) {
  assert(code != kOk);
  const uint32_t len1 = static_cast<uint32_t>(msg.size());
  const uint32_t len2 = static_cast<uint32_t>(msg2.size());
  const uint32_t size = len1 + (len2 ? (len2 + 2) : 0);

  char *result = new char[size + 5];
  std::memcpy(result, &size, sizeof(size));
  result[4] = static_cast<char>(code);
  std::memcpy(result + 5, msg.data(), len1);
  if (len2) {
    result[5 + len1] = ':';
    result[6 + len1] = ' ';
  }
  state_ = result;
}

} // namespace leveldb
