cmake_minimum_required(VERSION 3.28)
project(CAT LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# C standard for mixed C/C++ project
set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Find SDK path for Mac
if(APPLE)
    execute_process(
        COMMAND xcrun --show-sdk-path
        OUTPUT_VARIABLE MACOS_SDK_PATH
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    set(CMAKE_OSX_SYSROOT ${MACOS_SDK_PATH})
endif()

add_library(cat STATIC)

target_sources(cat
        PUBLIC
        FILE_SET HEADERS FILES
        Cat/Core/Log/log.h
        PRIVATE
        Cat/Core/Log/log.c
)

target_include_directories(cat
        PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/src>
        $<INSTALL_INTERFACE:include>

        PRIVATE
        ${CMAKE_SOURCE_DIR}/src/a
)
