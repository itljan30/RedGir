add_library(
    RedGir_cpp SHARED
    src/engine.cpp
    src/timer.cpp
    src/color.cpp
    src/sprite.cpp
)

target_include_directories(
    RedGir_cpp 
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/_include
)

set_target_properties(
    RedGir_cpp PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)

add_dependencies(RedGir_cpp RedGir_rs)
target_link_libraries(RedGir_cpp PRIVATE ${REDGIR_LIB})
