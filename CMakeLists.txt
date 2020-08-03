cmake_minimum_required(VERSION 3.0 FATAL_ERROR)

project(ZeekPluginRustySyslog)

enable_testing()

include(ZeekPlugin)

add_subdirectory(src/parser)

zeek_plugin_begin(Demo RustySyslog)
zeek_plugin_cc(src/syslog.cc src/Plugin.cc)
zeek_plugin_bif(src/rustysyslog.bif)
zeek_plugin_dist_files(README CHANGES COPYING VERSION)
zeek_plugin_link_library(parser)
zeek_plugin_end()

add_dependencies(bro-plugin-Demo_RustySyslog parser)

file(STRINGS "${CMAKE_CURRENT_SOURCE_DIR}/VERSION" VERSION LIMIT_COUNT 1)

if ("${PROJECT_SOURCE_DIR}" STREQUAL "${CMAKE_SOURCE_DIR}")
    # Allows building rpm/deb packages via "make package" in build dir.
    include(ConfigurePackaging)
    ConfigurePackaging(${VERSION})
endif ()
