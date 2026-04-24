import 'package:powersync/powersync.dart';

final testSchema = const Schema([
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
]);

class TestDatabaseHelper {
  late PowerSyncDatabase db;

  Future<void> init() async {
    // PowerSyncDatabase might require a path or use path_provider.
    // If it fails in tests, we will need to mock path_provider.
    db = PowerSyncDatabase(schema: testSchema);
    await db.initialize();
  }

  Future<void> seedTenantRegistry(String id, String name, String organizationId) async {
    await db.execute(
      'INSERT INTO tenant_registry (id, name, organization_id) VALUES (?, ?, ?)',
      [id, name, organizationId],
    );
  }

  Future<void> clear() async {
    await db.execute('DELETE FROM tenant_registry');
    await db.execute('DELETE FROM agent_missions');
    await db.execute('DELETE FROM agent_memories');
  }
}
