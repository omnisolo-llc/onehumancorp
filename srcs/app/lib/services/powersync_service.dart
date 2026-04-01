import 'dart:convert';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:powersync/powersync.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:io';
import 'package:flutter/foundation.dart';

final powersyncUrl = !kIsWeb && Platform.isAndroid ? 'http://10.0.2.2:8081' : 'http://localhost:8081';
final backendUrl = !kIsWeb && Platform.isAndroid ? 'http://10.0.2.2:8080' : 'http://localhost:8080';

final schema = Schema(const <Table>[
  Table('agent_missions', [
    Column.text('status'),
    Column.text('payload'),
    Column.text('created_at'),
  ])
]);

class AppPowerSyncConnector extends PowerSyncBackendConnector {
  PowerSyncDatabase db;

  AppPowerSyncConnector({required this.db});

  @override
  Future<PowerSyncCredentials?> fetchCredentials() async {
    final prefs = await SharedPreferences.getInstance();
    final jwt = prefs.getString('ohc_jwt');
    if (jwt == null) return null;

    final response = await http.get(
      Uri.parse('$backendUrl/api/powersync/token'),
      headers: {'Authorization': 'Bearer $jwt'},
    );

    if (response.statusCode == 200) {
      final body = jsonDecode(response.body);
      return PowerSyncCredentials(
        endpoint: body['powersync_url'] ?? powersyncUrl,
        token: body['token']
      );
    }
    return null;
  }

  @override
  Future<void> uploadData(PowerSyncDatabase database) async {
    final transaction = await database.getNextCrudTransaction();
    if (transaction == null) {
      return;
    }

    try {
      final prefs = await SharedPreferences.getInstance();
      final jwt = prefs.getString('ohc_jwt');
      if (jwt != null) {
        final bodyData = [];
        for (var op in transaction.crud) {
          bodyData.add({
            'op': op.op.name,
            'data': op.opData,
          });
        }

        final response = await http.post(
          Uri.parse('$backendUrl/api/powersync/upload'),
          headers: {
            'Authorization': 'Bearer $jwt',
            'Content-Type': 'application/json',
          },
          body: jsonEncode({
            'transaction': {
              'crud': bodyData
            }
          }),
        );

        if (response.statusCode == 200) {
          await transaction.complete();
        }
      }
    } catch (e) {
      // Error handling
    }
  }
}

class PowerSyncService {
  late PowerSyncDatabase db;
  late AppPowerSyncConnector connector;

  Future<void> init(bool isStandalone) async {
    if (isStandalone) {
      return;
    }

    final dir = await getApplicationDocumentsDirectory();
    final dbPath = join(dir.path, 'ohc_powersync.db');

    db = PowerSyncDatabase(
      schema: schema,
      path: dbPath,
    );

    await db.initialize();

    connector = AppPowerSyncConnector(db: db);
    db.connect(connector: connector);
  }
}

final powerSyncServiceProvider = Provider<PowerSyncService>((ref) {
  return PowerSyncService();
});
