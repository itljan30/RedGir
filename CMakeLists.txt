cmake_minimum_required(VERSION 3.15)
project(RedGir_cpp)

find_program(CARGO_EXE NAMES cargo)

if(NOT CARGO_EXE)
    message(FATAL_MESSAGE "Rust Cargo is not found, please install it to build RedGir")
endif()

if(NOT DEFINED CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

string(TOLOWER "${CMAKE_BUILD_TYPE}" RUST_BUILD_TYPE)
if(RUST_BUILD_TYPE STREQUAL "release")
    set(REDGIR_LIB ${CMAKE_CURRENT_SOURCE_DIR}/target/release/${CMAKE_SHARED_LIBRARY_PREFIX}redgir${CMAKE_SHARED_LIBRARY_SUFFIX})
    set(CARGO_COMMAND ${CARGO_EXE} build --features cpp-bindings --release)
else()
    set(REDGIR_LIB ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/${CMAKE_SHARED_LIBRARY_PREFIX}redgir${CMAKE_SHARED_LIBRARY_SUFFIX})
    set(CARGO_COMMAND ${CARGO_EXE} build --features cpp-bindings)
endif()

add_custom_command(
    OUTPUT ${REDGIR_LIB}
    COMMAND ${CARGO_COMMAND}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building RedGir Rust library"
)

add_custom_target(
    RedGir_rs
    DEPENDS ${REDGIR_LIB}
)

add_subdirectory(src/ffi/cpp)
