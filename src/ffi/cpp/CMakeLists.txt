if(DEFINED CMAKE_BUILD_TYPE)
    string(TOLOWER "${CMAKE_BUILD_TYPE}" BUILD_TYPE)
    if(BUILD_TYPE STREQUAL "release")
        set(REDGIR_DIR ${CMAKE_SOURCE_DIR}/target/release)
    else()
        set(REDGIR_DIR ${CMAKE_SOURCE_DIR}/target/debug)
    endif()
else()
    set(REDGIR_DIR ${CMAKE_SOURCE_DIR}/target/debug)
endif()

find_library(REDGIR NAMES redgir PATHS ${REDGIR_DIR} NO_DEFAULT_PATH)

if(NOT REDGIR)
    message(FATAL_ERROR "Could not find RedGir in ${REDGIR_DIR}")
endif()

add_library(
    RedGir_cpp STATIC
    src/engine.cpp
    src/timer.cpp
    src/color.cpp
    src/sprite.cpp
)

target_include_directories(
    RedGir_cpp 
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/RedGir
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/_include
)

set_target_properties(
    RedGir_cpp PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)

target_link_libraries(RedGir_cpp PRIVATE ${REDGIR})
