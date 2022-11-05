#include "status.h"

namespace leveldb {
class DB {
public:
  static Status Open(const std::string &name, DB **dbptr);
  DB() = default;
  DB(const DB &) = delete;
  DB &operator=(const DB &) = delete;
  virtual ~DB();

  virtual Status Put(const Slice &key, const Slice &value) = 0;
  virtual Status Get(const Slice &key, std::string *value) = 0;
};
} // namespace leveldb
