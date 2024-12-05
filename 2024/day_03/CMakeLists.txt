cmake_minimum_required(VERSION 3.31)

# Set the project name
project(Day03)

# Add the executable
add_executable(day03 main.cc ParserFSA.cc)

set(CMAKE_INSTALL_PREFIX ${CMAKE_SOURCE_DIR})
install(TARGETS day03 DESTINATION bin)
