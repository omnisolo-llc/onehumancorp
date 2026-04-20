import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:path/path.dart' as p;

/// Manages the local OpenClaw service and desktop-specific operations.
///
/// Desktop-only capabilities (filesystem, screenshots, process management)
/// are exposed here. The web/stub implementation no-ops all these methods.
class LocalManagerService {
  LocalManagerService({String? homeOverride}) : _homeOverride = homeOverride;

  final String? _homeOverride;

  Directory get _openclawDir {
    final home =
        _homeOverride ??
        Platform.environment['HOME'] ??
        Platform.environment['USERPROFILE'] ??
        '.';
    return Directory(p.join(home, '.openclaw'));
  }

  File get _configFile => File(p.join(_openclawDir.path, 'openclaw.json'));
  File get _envFile => File(p.join(_openclawDir.path, '.env'));

  // ─── Service lifecycle ───────────────────────────────────────────────────

  Future<Process> processStart(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
  }) {
    return Process.start(executable, arguments, runInShell: runInShell);
  }

  Future<ProcessResult> processRun(
    String executable,
    List<String> arguments, {
    bool runInShell = true,
  }) {
    return Process.run(executable, arguments, runInShell: runInShell);
  }

  Future<bool> isServiceRunning() async {
    try {
      final socket = await Socket.connect(
        'localhost',
        18789,
        timeout: const Duration(milliseconds: 500),
      );
      socket.destroy();
      return true;
    } catch (_) {
      return false;
    }
  }

  Future<void> startService() async {
    if (await isServiceRunning()) return;
    await processStart('ohc', ['start', '--daemon']);
  }

  Future<void> stopService() async {
    await processRun('ohc', ['stop']);
  }

  Future<void> restartService() async {
    await stopService();
    await startService();
  }

  // ─── Config / env ────────────────────────────────────────────────────────

  Future<Map<String, dynamic>> readConfig() async {
    if (!await _configFile.exists()) {
      return {};
    }
    final content = await _configFile.readAsString();
    return jsonDecode(content) as Map<String, dynamic>;
  }

  Future<void> writeConfig(Map<String, dynamic> config) async {
    if (!await _openclawDir.exists()) {
      await _openclawDir.create(recursive: true);
    }
    const encoder = JsonEncoder.withIndent('  ');
    await _configFile.writeAsString(encoder.convert(config));
  }

  Future<String?> getEnvValue(String key) async {
    if (!await _envFile.exists()) return null;
    final lines = await _envFile.readAsLines();
    for (final line in lines) {
      if (line.startsWith('$key=')) {
        return line.substring(key.length + 1).replaceAll('"', '').trim();
      }
    }
    return null;
  }

  Future<void> saveEnvValue(String key, String value) async {
    if (!await _openclawDir.exists()) {
      await _openclawDir.create(recursive: true);
    }
    var lines = <String>[];
    if (await _envFile.exists()) {
      lines = await _envFile.readAsLines();
    }

    var found = false;
    for (var index = 0; index < lines.length; index++) {
      if (lines[index].startsWith('$key=')) {
        lines[index] = '$key="$value"';
        found = true;
        break;
      }
    }

    if (!found) {
      lines.add('$key="$value"');
    }

    await _envFile.writeAsString('${lines.join('\n')}\n');
  }

  Future<String> runDoctor() async {
    final result = await processRun('ohc', ['doctor']);
    return result.stdout.toString() + result.stderr.toString();
  }

  Future<Map<String, dynamic>> getSystemInfo() async {
    return {
      'os': Platform.operatingSystem,
      'os_version': Platform.operatingSystemVersion,
      'dart_version': Platform.version,
      'hostname': Platform.localHostname,
      'cpus': Platform.numberOfProcessors,
    };
  }

  // ─── Filesystem operations ───────────────────────────────────────────────

  /// List the contents of [directory]. Returns a list of entry maps with
  /// keys: `name`, `path`, `type` ("file" | "directory"), `size`, `modified`.
  Future<List<Map<String, dynamic>>> listDirectory(String directory) async {
    final dir = Directory(directory);
    if (!await dir.exists()) return [];
    final entries = <Map<String, dynamic>>[];
    await for (final entity in dir.list()) {
      final stat = await entity.stat();
      entries.add({
        'name': p.basename(entity.path),
        'path': entity.path,
        'type': stat.type == FileSystemEntityType.directory ? 'directory' : 'file',
        'size': stat.size,
        'modified': stat.modified.toIso8601String(),
      });
    }
    return entries;
  }

  /// Read the contents of [filePath] as a UTF-8 string.
  Future<String> readFile(String filePath) async {
    return File(filePath).readAsString();
  }

  /// Read the raw bytes of [filePath].
  Future<Uint8List> readFileBytes(String filePath) async {
    return File(filePath).readAsBytes();
  }

  /// Write [content] to [filePath], creating parent directories as needed.
  Future<void> writeFile(String filePath, String content) async {
    final file = File(filePath);
    await file.parent.create(recursive: true);
    await file.writeAsString(content);
  }

  /// Write raw [bytes] to [filePath], creating parent directories as needed.
  Future<void> writeFileBytes(String filePath, List<int> bytes) async {
    final file = File(filePath);
    await file.parent.create(recursive: true);
    await file.writeAsBytes(bytes);
  }

  /// Delete the file or directory at [path].
  /// If [recursive] is true and [path] is a directory, deletes all contents.
  Future<void> deletePath(String path, {bool recursive = false}) async {
    final type = await FileSystemEntity.type(path);
    if (type == FileSystemEntityType.directory) {
      await Directory(path).delete(recursive: recursive);
    } else if (type == FileSystemEntityType.file) {
      await File(path).delete();
    }
  }

  /// Copy [source] to [destination].
  Future<void> copyFile(String source, String destination) async {
    await File(source).copy(destination);
  }

  /// Move / rename [source] to [destination].
  Future<void> moveFile(String source, String destination) async {
    await File(source).rename(destination);
  }

  /// Create a directory at [path] including all intermediate directories.
  Future<void> createDirectory(String path) async {
    await Directory(path).create(recursive: true);
  }

  /// Returns the user's home directory path.
  String get homeDirectory =>
      Platform.environment['HOME'] ??
      Platform.environment['USERPROFILE'] ??
      '.';

  // ─── Screenshot ──────────────────────────────────────────────────────────

  /// Capture a screenshot of the entire desktop and return the PNG bytes.
  ///
  /// Uses platform-native tools:
  ///   - macOS: `screencapture -x -t png -`
  ///   - Linux: `import -window root png:-` (ImageMagick) or `scrot -`
  ///   - Windows: PowerShell Add-Type screenshot via `powershell`
  ///
  /// Returns `null` if the capture is not supported on the current platform.
  Future<Uint8List?> captureScreenshot() async {
    try {
      if (Platform.isMacOS) {
        final tmpFile = p.join(
          Directory.systemTemp.path,
          'ohc_screenshot_${DateTime.now().millisecondsSinceEpoch}.png',
        );
        final result = await processRun(
          'screencapture',
          ['-x', '-t', 'png', tmpFile],
        );
        if (result.exitCode == 0) {
          final bytes = await File(tmpFile).readAsBytes();
          await File(tmpFile).delete();
          return Uint8List.fromList(bytes);
        }
      } else if (Platform.isLinux) {
        // Try gnome-screenshot first, then scrot, then import (ImageMagick)
        for (final cmd in [
          ['gnome-screenshot', '-f', '/dev/stdout'],
          ['scrot', '-'],
          ['import', '-window', 'root', 'png:-'],
        ]) {
          final result = await processRun(cmd.first, cmd.sublist(1));
          if (result.exitCode == 0 &&
              (result.stdout as List).isNotEmpty) {
            return Uint8List.fromList(result.stdout as List<int>);
          }
        }
      } else if (Platform.isWindows) {
        final tmpFile = p.join(
          Directory.systemTemp.path,
          'ohc_screenshot_${DateTime.now().millisecondsSinceEpoch}.png',
        );
        final psScript = [
          'Add-Type -AssemblyName System.Windows.Forms;',
          '\$bmp = [System.Drawing.Bitmap]::new([System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Width,',
          '  [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Height);',
          '\$g = [System.Drawing.Graphics]::FromImage(\$bmp);',
          '\$g.CopyFromScreen(0,0,0,0,\$bmp.Size);',
          '\$bmp.Save("$tmpFile");',
        ].join(' ');
        final result = await processRun('powershell', ['-Command', psScript]);
        if (result.exitCode == 0) {
          final bytes = await File(tmpFile).readAsBytes();
          await File(tmpFile).delete();
          return Uint8List.fromList(bytes);
        }
      }
    } catch (_) {
      // Screenshot not available; return null
    }
    return null;
  }

  /// Capture a screenshot and encode it as a base64 PNG data URI, suitable
  /// for embedding in an AI agent prompt or HTML.
  Future<String?> captureScreenshotAsDataUri() async {
    final bytes = await captureScreenshot();
    if (bytes == null) return null;
    return 'data:image/png;base64,${base64Encode(bytes)}';
  }

  // ─── Process management ──────────────────────────────────────────────────

  /// Launch an external application or command.
  ///
  /// Returns the [Process] handle for further interaction.
  Future<Process> openProcess(
    String executable,
    List<String> arguments, {
    String? workingDirectory,
    Map<String, String>? environment,
    bool runInShell = false,
  }) {
    return Process.start(
      executable,
      arguments,
      workingDirectory: workingDirectory,
      environment: environment,
      runInShell: runInShell,
    );
  }

  /// Open a URI with the system default handler (browser, file manager, etc.).
  ///
  /// On macOS uses `open`, on Linux `xdg-open`, on Windows `start`.
  Future<void> openUri(String uri) async {
    if (Platform.isMacOS) {
      await processRun('open', [uri]);
    } else if (Platform.isLinux) {
      await processRun('xdg-open', [uri]);
    } else if (Platform.isWindows) {
      await processRun('cmd', ['/c', 'start', '', uri]);
    }
  }

  /// Kill the process with the given [pid].
  Future<void> killProcess(int pid, {ProcessSignal signal = ProcessSignal.sigterm}) async {
    Process.killPid(pid, signal);
  }

  /// List running processes visible to the current user.
  ///
  /// Returns a list of maps with keys: `pid`, `name`, `cpu`, `memory`.
  /// Platform-specific; returns an empty list if unsupported.
  Future<List<Map<String, dynamic>>> getRunningProcesses() async {
    try {
      if (Platform.isMacOS || Platform.isLinux) {
        final result = await processRun(
          'ps',
          ['-eo', 'pid,comm,%cpu,%mem', '--no-headers'],
          runInShell: false,
        );
        if (result.exitCode == 0) {
          return (result.stdout as String)
              .split('\n')
              .where((line) => line.trim().isNotEmpty)
              .map((line) {
                final parts = line.trim().split(RegExp(r'\s+'));
                return {
                  'pid': int.tryParse(parts[0]) ?? 0,
                  'name': parts.length > 1 ? parts[1] : '',
                  'cpu': parts.length > 2 ? double.tryParse(parts[2]) ?? 0.0 : 0.0,
                  'memory': parts.length > 3 ? double.tryParse(parts[3]) ?? 0.0 : 0.0,
                };
              })
              .toList();
        }
      } else if (Platform.isWindows) {
        final result = await processRun(
          'tasklist',
          ['/fo', 'csv', '/nh'],
          runInShell: true,
        );
        if (result.exitCode == 0) {
          return (result.stdout as String)
              .split('\n')
              .where((line) => line.trim().isNotEmpty)
              .map((line) {
                final parts = line.split(',').map((s) => s.trim().replaceAll('"', '')).toList();
                return {
                  'pid': parts.length > 1 ? int.tryParse(parts[1]) ?? 0 : 0,
                  'name': parts.isNotEmpty ? parts[0] : '',
                  'cpu': 0.0,
                  'memory': 0.0,
                };
              })
              .toList();
        }
      }
    } catch (_) {
      // Process listing not available
    }
    return [];
  }
}

final localManagerServiceProvider = Provider((ref) => LocalManagerService());

final standaloneManagerProvider = Provider<void>((ref) {
  final settings = ref.watch(clientSettingsProvider).valueOrNull;
  if (settings == null || !settings.standaloneMode) return;

  final manager = ref.read(localManagerServiceProvider);
  manager.startService();
});