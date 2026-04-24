/// Abstract interface for desktop-platform-specific operations.
///
/// This interface exposes operations that are only meaningful on a native
/// desktop target (Windows, Linux, macOS) but are referenced in a
/// platform-agnostic way from the rest of the codebase.
///
/// Two concrete implementations are provided:
///   • `desktop_platform_service_io.dart`   – for dart:io platforms
///   • `desktop_platform_service_stub.dart` – no-op stub for web / unknown
abstract class DesktopPlatformService {
  // ── Process management ────────────────────────────────────────────────────

  /// Starts a new process and returns a handle to it.
  Future<ProcessHandle> startProcess(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
    Map<String, String>? environment,
    String? workingDirectory,
  });

  /// Runs a process to completion and returns its result.
  Future<ProcessResult> runProcess(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
    Map<String, String>? environment,
    String? workingDirectory,
  });

  // ── Filesystem operations ─────────────────────────────────────────────────

  /// Returns the user's home directory path.
  Future<String> homeDirectory();

  /// Lists the contents of [directory].
  ///
  /// Returns a list of [FileSystemEntry] values.  Non-existent directories
  /// return an empty list rather than throwing.
  Future<List<FileSystemEntry>> listDirectory(
    String directory, {
    bool recursive = false,
  });

  /// Reads the text content of a file.  Returns `null` if the file does not
  /// exist.
  Future<String?> readFile(String path);

  /// Writes [content] to [path], creating intermediate directories as needed.
  Future<void> writeFile(String path, String content);

  /// Deletes the file or directory at [path].  Directories are deleted
  /// recursively when [recursive] is `true`.
  Future<void> deleteFileSystemEntity(String path, {bool recursive = false});

  /// Returns `true` when the file or directory at [path] exists.
  Future<bool> exists(String path);

  /// Creates the directory at [path] (including any parent directories).
  Future<void> createDirectory(String path);

  /// Opens [path] using the platform's default application.
  Future<void> openPath(String path);

  // ── Screenshot ────────────────────────────────────────────────────────────

  /// Captures a screenshot of the primary display and writes it to a file.
  ///
  /// Returns the absolute path of the saved image, or `null` if the
  /// operation is not supported on the current platform.
  Future<String?> captureScreenshot({String? outputPath});

  /// Captures a screenshot and returns the raw PNG bytes directly, without
  /// writing to disk.  Useful for passing images to an AI agent.
  ///
  /// Returns `null` if screenshots are not supported on the current platform.
  Future<List<int>?> captureScreenshotBytes();

  // ── System information ────────────────────────────────────────────────────

  /// Returns information about the host operating system.
  Future<SystemInfo> getSystemInfo();
}

// ── Value objects ─────────────────────────────────────────────────────────────

/// A handle to a running process.
class ProcessHandle {
  const ProcessHandle({
    required this.pid,
    required this.stdin,
    required this.stdout,
    required this.stderr,
  });

  final int pid;

  /// Sink for data written to the process's standard-input stream.
  final Object? stdin; // Stream<List<int>> or equivalent

  /// Broadcast stream of data written to the process's standard-output stream.
  final Object? stdout; // Stream<List<int>> or equivalent

  /// Broadcast stream of data written to the process's standard-error stream.
  final Object? stderr; // Stream<List<int>> or equivalent
}

/// The result of a process that has run to completion.
class ProcessResult {
  const ProcessResult({
    required this.exitCode,
    required this.stdout,
    required this.stderr,
  });

  final int exitCode;
  final String stdout;
  final String stderr;

  bool get succeeded => exitCode == 0;
}

/// A single entry in a directory listing.
class FileSystemEntry {
  const FileSystemEntry({
    required this.path,
    required this.name,
    required this.isDirectory,
    this.sizeBytes,
    this.modifiedAt,
  });

  final String path;
  final String name;
  final bool isDirectory;
  final int? sizeBytes;
  final DateTime? modifiedAt;

  @override
  String toString() => 'FileSystemEntry(path: $path, isDirectory: $isDirectory)';
}

/// Summary of operating-system information.
class SystemInfo {
  const SystemInfo({
    required this.operatingSystem,
    required this.operatingSystemVersion,
    required this.hostname,
    required this.numberOfProcessors,
    required this.dartVersion,
  });

  final String operatingSystem;
  final String operatingSystemVersion;
  final String hostname;
  final int numberOfProcessors;
  final String dartVersion;

  Map<String, dynamic> toJson() => {
        'os': operatingSystem,
        'os_version': operatingSystemVersion,
        'hostname': hostname,
        'cpus': numberOfProcessors,
        'dart_version': dartVersion,
      };
}
