import 'package:powersync/powersync.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:sqlite_async/sqlite_async.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

import '../services/settings_service.dart';
import '../services/auth_service.dart';

const schema = Schema([
  Table(
    'agent_missions',
    [
      Column.text('id'),
      Column.text('status'),
      Column.text('payload'),
      Column.text('created_at'),
    ],
    indexes: [
      Index('status_idx', [IndexedColumn('status')]),
    ],
  ),
  // Map more tables here as needed
]);

class PowerSyncService {
  PowerSyncDatabase? db;
  final ClientSettings settings;
  final AuthService authService;

  PowerSyncService({required this.settings, required this.authService});

  Future<void> init() async {
    if (!settings.standaloneMode) {
      // In cloud-native or headless mode, use the API directly without local sync.
      return;
    }

    // In standalone mode, enable sync to cloud Postgres.
    final dir = await getApplicationDocumentsDirectory();
    final path = join(dir.path, 'powersync_swarm.db');

    db = PowerSyncDatabase(schema: schema, path: path);
    await db!.initialize();

    // In a real implementation we would conditionally connect if we have a cloud endpoint configured
    // For now we just implement the connector structure
    /*
    db!.connect(connector: _OHCBackendConnector(
        apiUrl: settings.backendUrl,
        authService: authService,
    ));
    */
  }
}

class _OHCBackendConnector extends PowerSyncBackendConnector {
  final String apiUrl;
  final AuthService authService;

  _OHCBackendConnector({required this.apiUrl, required this.authService});

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    final token = await authService.getToken();
    if (token == null) return null;

    try {
      final response = await http.get(
        Uri.parse('$apiUrl/api/powersync/token'),
        headers: {'Authorization': 'Bearer $token'},
      );

      if (response.statusCode == 200) {
        final data = json.decode(response.body);
        return PowerSyncCredentials(endpoint: apiUrl, token: data['token']);
      }
    } catch (e) {
      print('Failed to fetch PowerSync credentials: $e');
    }
    return null;
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // Implement upload logic to sync local changes to cloud
    // PowerSync relies on Postgres on the backend, but since we are using local SQLite,
    // this would be an implementation detail mapping PowerSync operations to our custom sync API.
  }
}
