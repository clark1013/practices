cmake_minimum_required(VERSION 3.10)
project(Game)
set(CMAKE_CXX_STANDARD, 11)

# add_library(2DGraphics1 GameLibs/2DGraphics1/src/Framework.h GameLibs/2DGraphics1/src/Framework.cpp)
# include_directories(GameLibs/include)
add_executable(NimotsuKun ch1/NimotsuKun/main.cpp)
add_executable(NimotsuKun2 ch1/NimotsuKun2/main.cpp)
