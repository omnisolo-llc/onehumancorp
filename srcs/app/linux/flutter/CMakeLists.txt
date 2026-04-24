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

target_compile_options(flutter INTERFACE
  "-include${EPHEMERAL_DIR}/flutter_export.h")
target_link_options(flutter INTERFACE "-Wl,-rpath,\$ORIGIN/lib")

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED IMPORTED_TARGET gtk+-3.0)
target_link_libraries(flutter INTERFACE PkgConfig::GTK)

function(apply_standard_settings TARGET)
  target_compile_options(${TARGET} PRIVATE -Wall -Werror)
  target_compile_options(${TARGET} PRIVATE
    "$<$<NOT:$<CONFIG:Debug>>:-O3>")
  target_compile_options(${TARGET} PRIVATE "$<$<CONFIG:Debug>:-g>")
  target_compile_definitions(${TARGET} PRIVATE
    "$<$<NOT:$<CONFIG:Debug>>:NDEBUG>")
endfunction()

set(PLUGIN_BUNDLED_LIBRARIES)

include(${CMAKE_CURRENT_SOURCE_DIR}/generated_plugins.cmake)
