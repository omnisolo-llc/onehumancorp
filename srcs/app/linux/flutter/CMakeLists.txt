cmake_minimum_required(VERSION 3.14)

set(EPHEMERAL_DIR "${CMAKE_CURRENT_SOURCE_DIR}/ephemeral")

if(EXISTS "${EPHEMERAL_DIR}/ephemeral.cmake")
  include(${EPHEMERAL_DIR}/ephemeral.cmake)
endif()
if(EXISTS "${EPHEMERAL_DIR}/generated_config.cmake")
  include(${EPHEMERAL_DIR}/generated_config.cmake)
endif()

set(FLUTTER_MANAGED_DIR ${CMAKE_CURRENT_SOURCE_DIR})

add_library(flutter INTERFACE)

set(FLUTTER_LIBRARY "${EPHEMERAL_DIR}/libflutter_linux_gtk.so")

add_custom_target(flutter_assemble DEPENDS "${FLUTTER_LIBRARY}")

if(NOT EXISTS "${FLUTTER_LIBRARY}")
  add_custom_command(
    OUTPUT "${FLUTTER_LIBRARY}"
    COMMAND ${CMAKE_COMMAND} -E copy_directory
      "${FLUTTER_ROOT}/bin/cache/artifacts/engine/${FLUTTER_TARGET_PLATFORM}"
      "${EPHEMERAL_DIR}"
    COMMENT "Copying Flutter Linux engine libraries"
  VERBATIM
  )
endif()

target_link_libraries(flutter INTERFACE "${FLUTTER_LIBRARY}")
target_include_directories(flutter INTERFACE "${EPHEMERAL_DIR}")
set(FLUTTER_LIBRARY_DIR "${EPHEMERAL_DIR}")

set(FLUTTER_ICU_DATA_FILE "${EPHEMERAL_DIR}/icudtl.dat")
set(FLUTTER_AOT_SNAPSHOT "${EPHEMERAL_DIR}/kernel_blob.bin")

set(FLUTTER_LIBRARY_DIR "${FLUTTER_LIBRARY_DIR}" PARENT_SCOPE)
set(FLUTTER_ICU_DATA_FILE "${FLUTTER_ICU_DATA_FILE}" PARENT_SCOPE)
set(FLUTTER_AOT_SNAPSHOT "${FLUTTER_AOT_SNAPSHOT}" PARENT_SCOPE)

target_link_options(flutter INTERFACE "-Wl,-rpath,\$ORIGIN/lib")

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk+-3.0)
target_link_libraries(flutter INTERFACE PkgConfig::GTK)

find_package(Python3 COMPONENTS Interpreter REQUIRED)

set(FLUTTER_PACKAGE_CONFIG "${CMAKE_CURRENT_SOURCE_DIR}/../../.dart_tool/package_config.json")
set(FLUTTER_PLUGIN_SYMLINK_DIR "${EPHEMERAL_DIR}/.plugin_symlinks")

if(EXISTS "${FLUTTER_PACKAGE_CONFIG}")
  file(MAKE_DIRECTORY "${FLUTTER_PLUGIN_SYMLINK_DIR}")
  execute_process(
    COMMAND
      "${Python3_EXECUTABLE}"
      "-c"
      [=[
import json
import pathlib
import shutil
import sys
import urllib.parse

config_path = pathlib.Path(sys.argv[1])
symlink_root = pathlib.Path(sys.argv[2])

if not config_path.is_file():
    raise SystemExit(0)

data = json.loads(config_path.read_text(encoding="utf-8"))
for package in data.get("packages", []):
    name = package.get("name")
    root_uri = package.get("rootUri")
    if not name or not root_uri:
        continue

    parsed = urllib.parse.urlparse(root_uri)
    decoded_path = urllib.parse.unquote(parsed.path or root_uri)
    root_path = pathlib.Path(decoded_path)
    if not root_path.is_absolute():
        root_path = (config_path.parent / root_path).resolve()
    if not (root_path / "linux").is_dir():
        continue

    link_path = symlink_root / name
    if link_path.is_symlink() or link_path.exists():
        if link_path.is_dir() and not link_path.is_symlink():
            shutil.rmtree(link_path)
        else:
            link_path.unlink()
    link_path.symlink_to(root_path)
]=]
      "${FLUTTER_PACKAGE_CONFIG}"
      "${FLUTTER_PLUGIN_SYMLINK_DIR}"
    RESULT_VARIABLE FLUTTER_PLUGIN_SYMLINK_RESULT
  )
  if(NOT FLUTTER_PLUGIN_SYMLINK_RESULT EQUAL 0)
    message(FATAL_ERROR "Failed to prepare Flutter Linux plugin symlinks")
  endif()
endif()

function(apply_standard_settings TARGET)
  target_compile_options(${TARGET} PRIVATE -Wall -Werror)
  target_compile_options(${TARGET} PRIVATE
    "$<$<NOT:$<CONFIG:Debug>>:-O3>")
  target_compile_options(${TARGET} PRIVATE "$<$<CONFIG:Debug>:-g>")
  target_compile_definitions(${TARGET} PRIVATE
    "$<$<NOT:$<CONFIG:Debug>>:NDEBUG>")
endfunction()

set(PLUGIN_BUNDLED_LIBRARIES "${PLUGIN_BUNDLED_LIBRARIES}" PARENT_SCOPE)
