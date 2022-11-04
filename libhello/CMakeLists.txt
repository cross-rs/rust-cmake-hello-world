cmake_minimum_required(VERSION 3.4)
project(hello)

add_library(hello hello.c)
target_include_directories(hello PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
install(TARGETS hello DESTINATION lib)
