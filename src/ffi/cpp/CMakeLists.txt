add_library(
    RedGir_cpp SHARED
    src/engine.cpp
    src/timer.cpp
    src/color.cpp
    src/sprite.cpp
)

# FIXME Make it so that functions inside _include do not clutter the autocomplete
target_include_directories(
    RedGir_cpp 
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include
)

add_dependencies(RedGir_cpp RedGir_rs)

if(MSVC)
    target_link_libraries(RedGir_cpp PRIVATE ${REDGIR_LIB}.lib)
else()
    target_link_libraries(RedGir_cpp PRIVATE ${REDGIR_LIB})
endif()
