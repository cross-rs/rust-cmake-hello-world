cmake_minimum_required(VERSION 3.4)
project(tryrun)

try_run(
    run_result
    compile_result
    "${CMAKE_CURRENT_BINARY_DIR}"
    "${CMAKE_CURRENT_SOURCE_DIR}/hello.c"
    RUN_OUTPUT_VARIABLE run_output
)

if(NOT compile_result)
    message(SEND_ERROR "unable to compile a simple hello world program.")
endif()

if(NOT run_result EQUAL 0)
    message(SEND_ERROR "unable to run a simple hello world program.")
endif()

if(NOT run_output MATCHES ".*Hello, World!.*")
    message(SEND_ERROR "unexpected out of \"${run_output}\" while running a simple hello world program.")
endif()

install(CODE "message(\"cmake-rs needs an install option.\")")
