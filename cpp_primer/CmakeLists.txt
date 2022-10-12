cmake_minimum_required(VERSION 3.10)

project(CppPrimer)

set(CMAKE_CXX_STANDARD, 11)

add_executable(io_tutorail ch1/io_tutorial.cpp)
add_executable(while_tutorail ch1/while_tutorial.cpp)
add_executable(for_tutorail ch1/for_tutorial.cpp)

add_executable(pow ch2/pow.cpp)

add_executable(pointer_test ch4/pointer_test.cpp)

add_executable(container ch9/container.cc)

add_executable(copy others/copy.cc)
