cmake_minimum_required(VERSION 3.28)
project(cat)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(cat STATIC)

target_sources(cat
    PUBLIC
        FILE_SET HEADERS FILES
            src/cat.h
        FILE_SET CXX_MODULES FILES
            src/cat/core/log/log.cppm
            src/cat/core/core.cppm
            src/cat/cat.cppm
)

target_include_directories(cat 
    PUBLIC 
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/src>
        $<INSTALL_INTERFACE:include>
)
