import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:powersync/powersync.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'package:ohc_app/services/auth_service.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

final _schema = Schema([
  const Table('agents', [
    Column.text('id'),
    Column.text('name'),
    Column.text('role'),
    Column.text('organization_id'),
    Column.text('status'),
    Column.text('provider_type'),
    Column.text('region'),
  ]),
  const Table('agent_inbox', [
    Column.text('message_id'),
    Column.text('agent_id'),
    Column.text('from_agent'),
    Column.text('to_agent'),
    Column.text('type'),
    Column.text('content'),
    Column.text('meeting_id'),
    Column.text('occurred_at'),
  ])
]);

class OHCBackendConnector extends PowerSyncBackendConnector {
  final Ref _ref;

  OHCBackendConnector(this._ref);

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    // The AsyncNotifierProvider exposes AsyncValue<AuthUser?>.
    final authUser = _ref.read(authStateProvider).valueOrNull;
    if (authUser == null) return null;

    final baseUrl = _ref.read(backendUrlProvider);

    // Request a PowerSync token from the Go server using the standard session token
    final response = await http.get(
      Uri.parse('$baseUrl/api/powersync/token'),
      headers: {'Authorization': 'Bearer ${authUser.token}'},
    );

    if (response.statusCode != 200) {
      throw Exception('Failed to fetch PowerSync token');
    }

    final body = jsonDecode(response.body);

    // In local dev, the backend runs on port 8080 or 18789 and PowerSync on 8081.
    // For a real deployment, this would be a specific POWERSYNC_URL.
    // For this task, we will try to substitute the port or append :8081 if it's localhost.
    final uri = Uri.parse(baseUrl);
    final String powerSyncUrl;
    if (uri.host == 'localhost' || uri.host == '127.0.0.1') {
      powerSyncUrl = '${uri.scheme}://${uri.host}:8081';
    } else {
      // Fallback for cloud mode
      powerSyncUrl = baseUrl.replaceAll('8080', '8081');
    }

    return PowerSyncCredentials(
      endpoint: powerSyncUrl,
      token: body['token'] as String,
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // This is a minimal implementation. Usually, we'd sync local changes via REST or an upload endpoint.
    // For this mission, read-only sync logic or basic architecture is sufficient.
    final transaction = await database.getNextCrudTransaction();
    if (transaction == null) return;
    await transaction.complete();
  }
}

class PowerSyncService {
  PowerSyncDatabase? _db;
  Future<PowerSyncDatabase>? _initFuture;
  final Ref _ref;

  PowerSyncService(this._ref);

  Future<PowerSyncDatabase> get db async {
    if (_db != null) return _db!;

    if (_initFuture != null) {
      return _initFuture!;
    }

    _initFuture = _initDb();
    return _initFuture!;
  }

  Future<PowerSyncDatabase> _initDb() async {
    final dir = await getApplicationDocumentsDirectory();
    final path = p.join(dir.path, 'powersync_ohc.db');

    final database = PowerSyncDatabase(
      schema: _schema,
      path: path,
    );

    await database.initialize();

    final connector = OHCBackendConnector(_ref);
    database.connect(connector: connector);

    _db = database;
    return _db!;
  }
}

final powerSyncServiceProvider = Provider<PowerSyncService>((ref) {
  return PowerSyncService(ref);
});
