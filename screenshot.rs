fn main() {
    slint::platform::set_platform(Box::new(slint::platform::software_renderer::MinimalSoftwareWindow::new(slint::platform::software_renderer::RepaintBufferType::NewBuffer)));
    // Need to use the macro but this is just a quick script if we could run it, however
    // building it needs Cargo or Bazel.
}
