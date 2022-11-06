cmake_minimum_required(VERSION 3.4)
project(sum)

add_library(sum sum.c)
target_include_directories(sum PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
install(TARGETS sum DESTINATION lib)

target_compile_features(sum PUBLIC cxx_std_11)
