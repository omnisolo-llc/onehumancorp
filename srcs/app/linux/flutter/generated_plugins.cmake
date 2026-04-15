#
# Generated file, do not edit.
#

list(APPEND FLUTTER_PLUGIN_LIST
  powersync_flutter_libs
  sqlite3_flutter_libs
)

list(APPEND FLUTTER_FFI_PLUGIN_LIST
)

set(PLUGIN_BUNDLED_LIBRARIES)

foreach(plugin ${FLUTTER_PLUGIN_LIST})
  if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/ephemeral/.plugin_symlinks/${plugin}/linux")
    add_subdirectory(flutter/ephemeral/.plugin_symlinks/${plugin}/linux plugins/${plugin})
    target_link_libraries(${BINARY_NAME} PRIVATE ${plugin}_plugin)
    list(APPEND PLUGIN_BUNDLED_LIBRARIES $<TARGET_FILE:${plugin}_plugin>)
    list(APPEND PLUGIN_BUNDLED_LIBRARIES ${${plugin}_bundled_libraries})
  endif()
endforeach(plugin)

foreach(ffi_plugin ${FLUTTER_FFI_PLUGIN_LIST})
  if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/ephemeral/.plugin_symlinks/${ffi_plugin}/linux")
    add_subdirectory(flutter/ephemeral/.plugin_symlinks/${ffi_plugin}/linux plugins/${ffi_plugin})
    list(APPEND PLUGIN_BUNDLED_LIBRARIES ${${ffi_plugin}_bundled_libraries})
  endif()
endforeach(ffi_plugin)
