import 'desktop_platform_service.dart';
export 'desktop_platform_service.dart';

/// Web / unsupported-platform stub for [DesktopPlatformService].
///
/// All operations return graceful no-ops or empty results rather than
/// throwing, so the rest of the codebase does not need platform-guards.
class DesktopPlatformServiceIO implements DesktopPlatformService {
  // ── Process management ────────────────────────────────────────────────────

  @override
  Future<ProcessHandle> startProcess(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
    Map<String, String>? environment,
    String? workingDirectory,
  }) async =>
      const ProcessHandle(pid: -1, stdin: null, stdout: null, stderr: null);

  @override
  Future<ProcessResult> runProcess(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
    Map<String, String>? environment,
    String? workingDirectory,
  }) async =>
      const ProcessResult(
        exitCode: -1,
        stdout: '',
        stderr: 'Process execution is not available on this platform.',
      );

  // ── Filesystem operations ─────────────────────────────────────────────────

  @override
  Future<String> homeDirectory() async => '/';

  @override
  Future<List<FileSystemEntry>> listDirectory(
    String directory, {
    bool recursive = false,
  }) async =>
      const [];

  @override
  Future<String?> readFile(String path) async => null;

  @override
  Future<void> writeFile(String path, String content) async {}

  @override
  Future<void> deleteFileSystemEntity(
    String path, {
    bool recursive = false,
  }) async {}

  @override
  Future<bool> exists(String path) async => false;

  @override
  Future<void> createDirectory(String path) async {}

  @override
  Future<void> openPath(String path) async {}

  // ── Screenshot ────────────────────────────────────────────────────────────

  @override
  Future<String?> captureScreenshot({String? outputPath}) async => null;

  @override
  Future<List<int>?> captureScreenshotBytes() async => null;

  // ── System information ────────────────────────────────────────────────────

  @override
  Future<SystemInfo> getSystemInfo() async => const SystemInfo(
        operatingSystem: 'web',
        operatingSystemVersion: 'unknown',
        hostname: 'unknown',
        numberOfProcessors: 1,
        dartVersion: 'unknown',
      );
}
