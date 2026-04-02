import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/settings_service.dart';

const _unsupportedMessage =
    'Local service management is not available in the web build.';

class LocalManagerService {
  LocalManagerService({String? homeOverride});

  Future<bool> isServiceRunning() async => false;

  Future<void> startService() async {}

  Future<void> stopService() async {}

  Future<void> restartService() async {}

  Future<Map<String, dynamic>> readConfig() async => {};

  Future<void> writeConfig(Map<String, dynamic> config) async {}

  Future<String?> getEnvValue(String key) async => null;

  Future<void> saveEnvValue(String key, String value) async {}

  Future<String> runDoctor() async => _unsupportedMessage;

  Future<Map<String, dynamic>> getSystemInfo() async {
    return {'os': 'web', 'status': _unsupportedMessage};
  }
}

final localManagerServiceProvider = Provider((ref) => LocalManagerService());

final standaloneManagerProvider = Provider<void>((ref) {
  final settings = ref.watch(clientSettingsProvider).valueOrNull;
  if (settings == null || !settings.standaloneMode) return;
});
