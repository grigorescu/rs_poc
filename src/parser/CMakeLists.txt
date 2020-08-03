if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

add_custom_target(parser_build ALL
    COMMENT "Compiling parser module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(parser_build PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})


find_library(PARSER_LIBRARY NAMES parser PATHS ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR} NO_DEFAULT_PATH)
add_library(parser SHARED IMPORTED GLOBAL)
set_property(TARGET parser PROPERTY IMPORTED_LOCATION "${PARSER_LIBRARY}")

add_dependencies(parser parser_build)

add_test(NAME parser_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
