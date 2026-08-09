# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target package_with_interfaces::package_with_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${package_with_interfaces_TARGETS}.
if(package_with_interfaces_TARGETS AND NOT TARGET package_with_interfaces::package_with_interfaces)
  add_library(package_with_interfaces::package_with_interfaces INTERFACE IMPORTED)
  set_target_properties(package_with_interfaces::package_with_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${package_with_interfaces_TARGETS}")
endif()
