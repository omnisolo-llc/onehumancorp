/// Stub implementation of the path_provider package for Bazel builds.
library path_provider;

import 'dart:io';

/// Returns the application support directory.
Future<Directory> getApplicationSupportDirectory() async {
  return Directory('/tmp/app_support');
}

/// Returns the application documents directory.
Future<Directory> getApplicationDocumentsDirectory() async {
  return Directory('/tmp/app_documents');
}

/// Returns the application cache directory.
Future<Directory> getApplicationCacheDirectory() async {
  return Directory('/tmp/app_cache');
}

/// Returns the temporary directory.
Future<Directory> getTemporaryDirectory() async {
  return Directory('/tmp');
}
