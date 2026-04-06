import 'dart:convert';
import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:path/path.dart' as p;

/// Manages the local OpenClaw service and its configuration.
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

    await _envFile.writeAsString(lines.join('\n') + '\n');
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
}

final localManagerServiceProvider = Provider((ref) => LocalManagerService());

final standaloneManagerProvider = Provider<void>((ref) {
  final settings = ref.watch(clientSettingsProvider).valueOrNull;
  if (settings == null || !settings.standaloneMode) return;

  final manager = ref.read(localManagerServiceProvider);
  manager.startService();
});
