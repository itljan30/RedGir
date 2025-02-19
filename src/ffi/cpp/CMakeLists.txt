cmake_minimum_required(VERSION 3.15)
project(RedGir_cpp)

add_library(RedGir_cpp STATIC
    src/engine.cpp
    src/timer.cpp
    src/color.cpp
    src/sprite.cpp
)
target_include_directories(RedGir_cpp PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)

install(TARGETS RedGir_cpp DESTINATION lib)
install(DIRECTORY include/ DESTINATION include)
