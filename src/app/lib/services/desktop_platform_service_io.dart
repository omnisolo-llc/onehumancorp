import 'dart:convert';
import 'dart:io';

import 'package:path/path.dart' as p;

import 'desktop_platform_service.dart';
export 'desktop_platform_service.dart';

/// dart:io implementation of [DesktopPlatformService].
///
/// Supports Windows, Linux, and macOS desktop targets.
class DesktopPlatformServiceIO implements DesktopPlatformService {
  // ── Process management ────────────────────────────────────────────────────

  @override
  Future<ProcessHandle> startProcess(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
    Map<String, String>? environment,
    String? workingDirectory,
  }) async {
    final process = await Process.start(
      executable,
      arguments,
      runInShell: runInShell,
      environment: environment,
      workingDirectory: workingDirectory,
    );
    return ProcessHandle(
      pid: process.pid,
      stdin: process.stdin,
      stdout: process.stdout,
      stderr: process.stderr,
    );
  }

  @override
  Future<ProcessResult> runProcess(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
    Map<String, String>? environment,
    String? workingDirectory,
  }) async {
    final result = await Process.run(
      executable,
      arguments,
      runInShell: runInShell,
      environment: environment,
      workingDirectory: workingDirectory,
    );
    return ProcessResult(
      exitCode: result.exitCode,
      stdout: result.stdout.toString(),
      stderr: result.stderr.toString(),
    );
  }

  // ── Filesystem operations ─────────────────────────────────────────────────

  @override
  Future<String> homeDirectory() async {
    return Platform.environment['HOME'] ??
        Platform.environment['USERPROFILE'] ??
        '.';
  }

  @override
  Future<List<FileSystemEntry>> listDirectory(
    String directory, {
    bool recursive = false,
  }) async {
    final dir = Directory(directory);
    if (!await dir.exists()) return [];
    final entries = <FileSystemEntry>[];
    await for (final entity in dir.list(recursive: recursive)) {
      final stat = await entity.stat();
      entries.add(FileSystemEntry(
        path: entity.path,
        name: p.basename(entity.path),
        isDirectory: entity is Directory,
        sizeBytes: stat.size > 0 ? stat.size : null,
        modifiedAt: stat.modified,
      ));
    }
    return entries;
  }

  @override
  Future<String?> readFile(String path) async {
    final file = File(path);
    if (!await file.exists()) return null;
    return file.readAsString();
  }

  @override
  Future<void> writeFile(String path, String content) async {
    final file = File(path);
    await file.parent.create(recursive: true);
    await file.writeAsString(content);
  }

  @override
  Future<void> deleteFileSystemEntity(
    String path, {
    bool recursive = false,
  }) async {
    final file = File(path);
    if (await file.exists()) {
      await file.delete();
      return;
    }
    final dir = Directory(path);
    if (await dir.exists()) {
      await dir.delete(recursive: recursive);
    }
  }

  @override
  Future<bool> exists(String path) async {
    return File(path).exists().then((v) => v ? true : Directory(path).exists());
  }

  @override
  Future<void> createDirectory(String path) async {
    await Directory(path).create(recursive: true);
  }

  @override
  Future<void> openPath(String path) async {
    if (Platform.isMacOS) {
      await Process.run('open', [path]);
    } else if (Platform.isLinux) {
      await Process.run('xdg-open', [path]);
    } else if (Platform.isWindows) {
      await Process.run('explorer', [path], runInShell: true);
    }
  }

  // ── Screenshot ────────────────────────────────────────────────────────────

  @override
  Future<String?> captureScreenshot({String? outputPath}) async {
    final outPath = outputPath ??
        p.join(
          (await homeDirectory()),
          'screenshot_${DateTime.now().millisecondsSinceEpoch}.png',
        );
    ProcessResult result;
    if (Platform.isMacOS) {
      result = await runProcess('screencapture', ['-x', '-t', 'png', outPath]);
    } else if (Platform.isLinux) {
      // Try scrot first, fall back to gnome-screenshot.
      result = await runProcess('scrot', [outPath]);
      if (result.exitCode != 0) {
        result = await runProcess(
          'gnome-screenshot',
          ['-f', outPath],
        );
      }
    } else if (Platform.isWindows) {
      // Use PowerShell to capture the screen on Windows.
      final psScript = '''
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Screen]::PrimaryScreen | ForEach-Object {
  \$bmp = New-Object System.Drawing.Bitmap(\$_.Bounds.Width, \$_.Bounds.Height)
  \$g = [System.Drawing.Graphics]::FromImage(\$bmp)
  \$g.CopyFromScreen(\$_.Bounds.Location, [System.Drawing.Point]::Empty, \$_.Bounds.Size)
  \$bmp.Save("$outPath", [System.Drawing.Imaging.ImageFormat]::Png)
}
''';
      result = await runProcess(
        'powershell',
        ['-NoProfile', '-Command', psScript],
      );
    } else {
      return null;
    }
    return result.succeeded ? outPath : null;
  }

  @override
  Future<List<int>?> captureScreenshotBytes() async {
    final tmpPath = p.join(
      (await homeDirectory()),
      '.ohc_screenshot_tmp_${DateTime.now().millisecondsSinceEpoch}.png',
    );
    final saved = await captureScreenshot(outputPath: tmpPath);
    if (saved == null) return null;
    try {
      final bytes = await File(saved).readAsBytes();
      await File(saved).delete();
      return bytes;
    } catch (_) {
      return null;
    }
  }

  // ── System information ────────────────────────────────────────────────────

  @override
  Future<SystemInfo> getSystemInfo() async => SystemInfo(
        operatingSystem: Platform.operatingSystem,
        operatingSystemVersion: Platform.operatingSystemVersion,
        hostname: Platform.localHostname,
        numberOfProcessors: Platform.numberOfProcessors,
        dartVersion: Platform.version,
      );
}
