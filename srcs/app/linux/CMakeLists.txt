cmake_minimum_required(VERSION 3.14)
project(runner LANGUAGES CXX)

set(BINARY_NAME "ohc_app")
set(APPLICATION_ID "com.onehumancorp.app")

cmake_policy(VERSION 3.14...3.25)

set(CMAKE_INSTALL_PREFIX "${CMAKE_BINARY_DIR}/install")
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(FLUTTER_MANAGED_DIR "${CMAKE_CURRENT_SOURCE_DIR}/flutter")
add_subdirectory(${FLUTTER_MANAGED_DIR} managed_flutter)

add_executable(${BINARY_NAME}
  "main.cc"
  "my_application.cc"
  "${FLUTTER_MANAGED_DIR}/generated_plugin_registrant.cc"
)

apply_standard_settings(${BINARY_NAME})

target_link_libraries(${BINARY_NAME} PRIVATE flutter)
target_include_directories(${BINARY_NAME} PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${BINARY_NAME} flutter_assemble)

set(PLUGIN_BUNDLED_LIBRARIES)
include(${FLUTTER_MANAGED_DIR}/generated_plugins.cmake)

install(TARGETS ${BINARY_NAME} RUNTIME DESTINATION "${CMAKE_INSTALL_PREFIX}")
install(FILES "${FLUTTER_ICU_DATA_FILE}" DESTINATION "${CMAKE_INSTALL_PREFIX}/data")
install(FILES "${FLUTTER_AOT_SNAPSHOT}" DESTINATION "${CMAKE_INSTALL_PREFIX}/data")
install(DIRECTORY "${FLUTTER_LIBRARY_DIR}" DESTINATION "${CMAKE_INSTALL_PREFIX}")
install(DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/plugins"
  DESTINATION "${CMAKE_INSTALL_PREFIX}"
)

foreach(bundled_library ${PLUGIN_BUNDLED_LIBRARIES})
  install(FILES "${bundled_library}"
    DESTINATION "${CMAKE_INSTALL_PREFIX}/lib"
    COMPONENT Runtime)
endforeach(bundled_library)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
