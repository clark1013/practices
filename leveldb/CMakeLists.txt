cmake_minimum_required(VERSION 3.9)
set(CMAKE_CXX_STANDARD 11)

project(leveldb)

add_library(leveldb "")
target_sources(leveldb
  PRIVATE
    "db/status.cc"
	"db/arena.cc"
  $<$<VERSION_GREATER:CMAKE_VERSION,3.2>:PUBLIC>
    "db/status.h"
	"db/arena.h"
)

add_subdirectory("third_party/googletest")

add_executable(leveldb_tests
  "db/status_test.cc"
  "db/arena_test.cc"
  "db/run_test.cc"
)
target_link_libraries(leveldb_tests leveldb gtest)
