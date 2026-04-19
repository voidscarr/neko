cmake_minimum_required(VERSION 3.19)
project(neko LANGUAGES CXX)

SET(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Core Widgets LinguistTools)

add_library(neko_core SHARED IMPORTED)
set(NEKO_CORE_LIB "${CMAKE_SOURCE_DIR}/../core/target/debug/libneko_core.a")
set_target_properties(neko_core PROPERTIES 
  IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/../core/target/cxxbridge/neko_core/src/ffi.rs.cc"
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_SOURCE_DIR}/../core/target/cxxbridge/neko_core/src"
)

qt_standard_project_setup()

qt_add_executable(neko
    WIN32 MACOSX_BUNDLE
    main.cpp
    features/window/window.cpp
    features/window/window.h

    features/editor/editor_widget.cpp
    features/editor/editor_widget.h
)

qt_add_translations(
    TARGETS neko
    TS_FILES neko_en_US.ts
)

target_link_libraries(neko
    PRIVATE
        Qt::Core
        Qt::Widgets
        neko_core
        ${NEKO_CORE_LIB}
)

include(GNUInstallDirs)

install(TARGETS neko
    BUNDLE  DESTINATION .
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)

qt_generate_deploy_app_script(
    TARGET neko
    OUTPUT_SCRIPT deploy_script
    NO_UNSUPPORTED_PLATFORM_ERROR
)
install(SCRIPT ${deploy_script})
