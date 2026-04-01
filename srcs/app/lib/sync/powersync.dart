import 'package:flutter/foundation.dart';
import 'package:powersync/powersync.dart';

final powersync = PowerSyncDatabase(
  schema: const Schema([
    Table('agents', [
      Column.text('name'),
      Column.text('role'),
      Column.text('organization_id'),
    ]),
    Table('agent_missions', [
      Column.text('status'),
      Column.text('payload'),
    ]),
    Table('meeting_rooms', [
      Column.text('agenda'),
    ]),
  ]),
  maxReaders: 3,
);

class Connector extends PowerSyncBackendConnector {
  final String backendUrl;
  final String token;

  Connector({required this.backendUrl, required this.token});

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    return PowerSyncCredentials(
      endpoint: '$backendUrl/powersync',
      token: token,
    );
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // Note: Hybrid syncing local client overriding
  }
}

Future<void> initPowerSync(String backendUrl, String jwtToken) async {
  if (kIsWeb) return; // Currently avoiding complex web init, fallback appropriately

  await powersync.initialize();
  powersync.connect(connector: Connector(backendUrl: backendUrl, token: jwtToken));
}
