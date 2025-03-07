cmake_minimum_required(VERSION 3.15)
project(RedGir_cpp)

if(NOT DEFINED CMAKE_LIBRARY_OUTPUT_DIRECTORY)
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target)
endif()

find_program(CARGO_EXE NAMES cargo)

if(NOT CARGO_EXE)
    message(FATAL_MESSAGE "Rust Cargo is not found, please install it to build RedGir")
endif()

if(NOT DEFINED CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

string(TOLOWER "${CMAKE_BUILD_TYPE}" RUST_BUILD_TYPE)
if(RUST_BUILD_TYPE STREQUAL "release")
    set(BUILD_TYPE release)
    set(CARGO_COMMAND ${CARGO_EXE} build --features cpp-bindings --release)
else()
    set(BUILD_TYPE debug)
    set(CARGO_COMMAND ${CARGO_EXE} build --features cpp-bindings)
endif()

set(REDGIR_LIB ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/${BUILD_TYPE}/${CMAKE_SHARED_LIBRARY_PREFIX}redgir${CMAKE_SHARED_LIBRARY_SUFFIX})

add_custom_command(
    OUTPUT ${REDGIR_LIB}
    COMMAND ${CARGO_COMMAND} --target-dir ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building RedGir Rust library"
)

add_custom_target(
    RedGir_rs
    DEPENDS ${REDGIR_LIB}
)

add_subdirectory(src/ffi/cpp)
