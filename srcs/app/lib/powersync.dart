import 'package:flutter/foundation.dart';
import 'package:powersync/powersync.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:io';

late final PowerSyncDatabase db;

const schema = Schema([
  Table('agent_missions', [
    Column.text('status'),
    Column.text('payload'),
    Column.text('created_at'),
  ]),
  Table('users', [
    Column.text('email'),
    Column.text('name'),
  ]),
  Table('meetings', [
    Column.text('title'),
    Column.text('status'),
  ]),
]);

class BackendConnector extends PowerSyncBackendConnector {
  final String backendUrl;

  BackendConnector(this.backendUrl);

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    try {
      final response = await http.get(Uri.parse('$backendUrl/api/auth/token'));
      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        return PowerSyncCredentials(
          endpoint: '$backendUrl/api/powersync',
          token: data['token'] as String,
        );
      }
    } catch (e) {
      debugPrint('Error fetching credentials: \$e');
    }
    return null;
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    // In a full implementation, you'd send pending mutations to the cloud backend.
  }
}

Future<void> initPowerSync({String backendUrl = 'http://127.0.0.1:8080', bool isStandalone = false}) async {
  String path = 'powersync-local.db';

  if (!kIsWeb) {
    final dir = await getApplicationDocumentsDirectory();
    final ohcDir = Directory(p.join(dir.path, '.openclaw'));
    if (!ohcDir.existsSync()) {
      ohcDir.createSync(recursive: true);
    }
    path = p.join(ohcDir.path, 'powersync-local.db');
  }

  db = PowerSyncDatabase(schema: schema, path: path);
  await db.initialize();

  if (!isStandalone) {
    db.connect(connector: BackendConnector(backendUrl));
  }
}
