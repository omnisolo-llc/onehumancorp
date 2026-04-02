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
    // Wait for settings to load properly
    // Since StateNotifier doesn't expose a future directly we can listen to it.
    // A more robust way to wait for initialization is to use the shared preferences future.
    // However, we can use the provider directly if we subscribe or wait until it's not loading.

    // In this specific architecture, settings are usually loaded very quickly via SharedPreferences.
    // For initializing async services like PowerSync, it's safer to wait for the value if it's loading.
    // Given the constraints of the current provider setup (StateNotifierProvider<_, AsyncValue>),
    // we can use a Completer if we wanted to listen, or simply read the prefs directly since it's an init method.

    // A simpler and more direct approach is to just await the SharedPreferences to ensure they're loaded,
    // then read the state again, or just let the provider do its thing and only proceed if data is available.
    // Since PowerSync initialization happens once, we will ensure we get the data.

    var settingsState = _ref.read(clientSettingsProvider);
    while (settingsState.isLoading) {
        await Future.delayed(const Duration(milliseconds: 10));
        settingsState = _ref.read(clientSettingsProvider);
    }

    final settings = settingsState.valueOrNull;

    // Only initialize PowerSync in Standalone mode
    if (settings == null || !settings.standaloneMode) {
      return;
    }

    const schema = Schema([
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
    ]);

    final dir = await getApplicationSupportDirectory();
    final path = p.join(dir.path, 'powersync.db');

    _db = PowerSyncDatabase(schema: schema, path: path);
    await _db!.initialize();

    final backendUrl = settings.backendUrl;

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
    final authUser = ref.read(authStateProvider).valueOrNull;
    if (authUser == null) {
      return null;
    }

    return PowerSyncCredentials(
      endpoint: backendUrl,
      token: authUser.token,
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // Implement upload data logic if local modifications are allowed.
    // In hybrid setup, SQLite is mostly for reading synced data,
    // but you could add API calls here to sync local changes up.
  }
}
