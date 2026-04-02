import 'dart:io';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:powersync/powersync.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/auth_service.dart';

final powersyncProvider = Provider<PowerSyncService>((ref) {
  final service = PowerSyncService(ref);
  ref.onDispose(() => service.dispose());
  return service;
});

class PowerSyncService {
  final Ref _ref;
  PowerSyncDatabase? _db;
  bool _initialized = false;

  PowerSyncService(this._ref) {
    _init();
  }

  Future<void> _init() async {
    final settings = await _ref.read(clientSettingsProvider.future);

    // Only initialize PowerSync in Standalone mode
    if (!settings.standaloneMode) {
      return;
    }

    final schema = Schema((<Table>[
      Table('agents', [
        Column.text('name'),
        Column.text('role'),
        Column.text('organization_id'),
        Column.text('status'),
        Column.text('provider_type'),
        Column.text('region'),
      ]),
      Table('meeting_rooms', [
        Column.text('agenda'),
        Column.text('participants'),
      ]),
      Table('agent_missions', [
        Column.text('status'),
        Column.text('payload'),
        Column.text('created_at'),
      ]),
      Table('swarm_memory', [
        Column.text('value'),
        Column.text('updated_at'),
      ], indexes: [
        Index('idx_swarm_memory_updated_at', [IndexedColumn('updated_at')])
      ]),
      Table('capability_plugins', [
        Column.text('name'),
        Column.text('version'),
        Column.text('manifest_url'),
        Column.text('status'),
        Column.text('registered_at'),
      ]),
      Table('swarm_memory_embeddings', [
        Column.text('context'),
        Column.text('source_plugin'),
        Column.text('created_at'),
      ]),
      Table('agent_status', [
        Column.text('role'),
        Column.text('status'),
        Column.text('last_heartbeat'),
      ]),
    ]));

    final dir = await getApplicationSupportDirectory();
    final path = p.join(dir.path, 'powersync.db');

    _db = PowerSyncDatabase(schema: schema, path: path);
    await _db!.initialize();

    final backendUrl = settings.serverUrl;

    PowerSyncBackendConnector connector = _BackendConnector(
      backendUrl: backendUrl,
      ref: _ref,
    );

    await _db!.connect(connector: connector);
    _initialized = true;
  }

  bool get isInitialized => _initialized;
  PowerSyncDatabase? get db => _db;

  void dispose() {
    _db?.disconnect();
  }
}

class _BackendConnector extends PowerSyncBackendConnector {
  final String backendUrl;
  final Ref ref;

  _BackendConnector({required this.backendUrl, required this.ref});

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    final token = ref.read(authServiceProvider).getToken();
    if (token == null) {
      return null;
    }

    return PowerSyncCredentials(
      endpoint: backendUrl,
      token: token,
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // Implement upload data logic if local modifications are allowed.
    // In hybrid setup, SQLite is mostly for reading synced data,
    // but you could add API calls here to sync local changes up.
  }
}
