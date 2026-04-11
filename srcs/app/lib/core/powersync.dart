import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:powersync/powersync.dart';

// Create a central PowerSyncDatabase instance
final PowerSyncDatabase powerSyncDb = PowerSyncDatabase(
  schema: const Schema([
    Table('tenant_registry', [
      Column.text('id'),
      Column.text('name'),
      Column.text('organization_id'),
    ]),
    Table('agent_missions', [
      Column.text('id'),
      Column.text('title'),
      Column.text('status'),
      Column.text('organization_id'),
    ]),
    Table('agent_memories', [
      Column.text('id'),
      Column.text('content'),
      Column.text('organization_id'),
    ]),
  ]),
  path: 'powersync.db',
);

class BackendConnector extends PowerSyncBackendConnector {
  final String backendUrl;
  final String sessionToken;

  BackendConnector(this.backendUrl, this.sessionToken);

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    // Call our newly created endpoint
    final response = await http.get(
      Uri.parse('$backendUrl/api/auth/powersync/token'),
      headers: {
        'Authorization': 'Bearer $sessionToken',
      },
    );

    if (response.statusCode != 200) {
      throw Exception('Failed to fetch PowerSync token: ${response.statusCode}');
    }

    final data = jsonDecode(response.body);

    return PowerSyncCredentials(
      endpoint: data['power_sync_url'] ?? 'http://localhost:8081',
      token: data['token'],
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // In a full implementation, we'd batch upload local modifications here.
    // However, our backend has HTTP endpoints for regular actions, so we rely on them
    // for direct writes, and let PowerSync sync the results back down.
    // For purely offline writes, we would iterate over database.getCrudBatch() here.
  }
}
