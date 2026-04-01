import 'package:powersync/powersync.dart';
import 'package:sqlite_async/sqlite_async.dart';

final powerSync = PowerSyncDatabase(
  schema: const Schema([
    Table('swarm_memory', [
      Column.text('value'),
      Column.text('updated_at'),
    ]),
    Table('agent_missions', [
      Column.text('status'),
      Column.text('payload'),
      Column.text('created_at'),
    ]),
  ]),
  maxReaders: 3,
);

class SupabaseConnector extends PowerSyncBackendConnector {
  final String token;

  SupabaseConnector(this.token);

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    return PowerSyncCredentials(endpoint: 'http://localhost:8080', token: token);
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // Implementation for dynamic syncing
  }
}
