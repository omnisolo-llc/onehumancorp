import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:powersync/powersync.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/auth_service.dart';

final powerSyncProvider = Provider<PowerSyncService>((ref) {
  return PowerSyncService(ref);
});

class PowerSyncService {
  final Ref _ref;
  late PowerSyncDatabase db;

  PowerSyncService(this._ref) {
    db = PowerSyncDatabase(
      schema: _schema,
      path: 'powersync.db',
    );
  }

  Future<void> init() async {
    // Dynamic initialization based on mode (Standalone vs Cloud).
    // Sync to local SQLite when appropriate.
    await db.initialize();

    final token = await _fetchToken();
    if (token != null) {
      db.connect(connector: OhcConnector(_ref));
    }
  }

  Future<String?> _fetchToken() async {
    final settingsAsync = _ref.read(clientSettingsProvider);
    final settings = settingsAsync.value;
    if (settings == null) return null;

    final authState = _ref.read(authStateProvider);
    final token = authState.valueOrNull?.token;
    if (token == null) return null;

    final baseUrl = settings.backendUrl;
    try {
      final res = await http.get(
        Uri.parse('$baseUrl/api/powersync/token'),
        headers: {'Authorization': 'Bearer $token'},
      );
      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        return data['token'];
      }
    } catch (_) {}
    return null;
  }
}

class OhcConnector extends PowerSyncBackendConnector {
  final Ref _ref;

  OhcConnector(this._ref);

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    final settingsAsync = _ref.read(clientSettingsProvider);
    final settings = settingsAsync.value;
    if (settings == null) return null;

    final authState = _ref.read(authStateProvider);
    final token = authState.valueOrNull?.token;
    if (token == null) return null;

    final baseUrl = settings.backendUrl;
    var powerSyncUrl = baseUrl.replaceAll('8080', '8081');

    try {
      final res = await http.get(
        Uri.parse('$baseUrl/api/powersync/token'),
        headers: {'Authorization': 'Bearer $token'},
      );
      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        return PowerSyncCredentials(
          endpoint: powerSyncUrl,
          token: data['token'],
        );
      }
    } catch (_) {}
    return null;
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // Unidirectional sync for now
  }
}

const _schema = Schema([
  Table('agent_missions', [
    Column.text('title'),
    Column.text('description'),
    Column.text('status'),
  ])
]);
