import 'dart:typed_data';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/settings_service.dart';

const _unsupportedMessage =
    'This operation is not available in the web build.';

/// Web/stub implementation of LocalManagerService.
/// All desktop-only operations are no-ops or return empty values.
class LocalManagerService {
  LocalManagerService({String? homeOverride});

  // ─── Service lifecycle ───────────────────────────────────────────────────

  Future<bool> isServiceRunning() async => false;
  Future<void> startService() async {}
  Future<void> stopService() async {}
  Future<void> restartService() async {}

  // ─── Config / env ────────────────────────────────────────────────────────

  Future<Map<String, dynamic>> readConfig() async => {};
  Future<void> writeConfig(Map<String, dynamic> config) async {}
  Future<String?> getEnvValue(String key) async => null;
  Future<void> saveEnvValue(String key, String value) async {}
  Future<String> runDoctor() async => _unsupportedMessage;
  Future<Map<String, dynamic>> getSystemInfo() async => {'os': 'web'};

  String get homeDirectory => '';

  // ─── Filesystem operations ───────────────────────────────────────────────

  Future<List<Map<String, dynamic>>> listDirectory(String directory) async => [];
  Future<String> readFile(String filePath) async => '';
  Future<Uint8List> readFileBytes(String filePath) async => Uint8List(0);
  Future<void> writeFile(String filePath, String content) async {}
  Future<void> writeFileBytes(String filePath, List<int> bytes) async {}
  Future<void> deletePath(String path, {bool recursive = false}) async {}
  Future<void> copyFile(String source, String destination) async {}
  Future<void> moveFile(String source, String destination) async {}
  Future<void> createDirectory(String path) async {}

  // ─── Screenshot ──────────────────────────────────────────────────────────

  Future<Uint8List?> captureScreenshot() async => null;
  Future<String?> captureScreenshotAsDataUri() async => null;

  // ─── Process management ──────────────────────────────────────────────────

  Future<void> openUri(String uri) async {}
  Future<void> killProcess(int pid) async {}
  Future<List<Map<String, dynamic>>> getRunningProcesses() async => [];
}

final localManagerServiceProvider = Provider((ref) => LocalManagerService());

final standaloneManagerProvider = Provider<void>((ref) {
  final settings = ref.watch(clientSettingsProvider).valueOrNull;
  if (settings == null || !settings.standaloneMode) return;
});