import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/services/local_manager_service.dart';


void main() {
  late Directory tempHome;
  late LocalManagerService service;

  setUp(() async {
    tempHome = await Directory.systemTemp.createTemp('ohc_local_manager_test_');
    service = LocalManagerService(homeOverride: tempHome.path);
  });

  tearDown(() async {
    if (await tempHome.exists()) {
      await tempHome.delete(recursive: true);
    }
  });

  test('readConfig returns empty map when config does not exist', () async {
    final cfg = await service.readConfig();
    expect(cfg, isEmpty);
  });

  test('writeConfig persists JSON and readConfig returns it', () async {
    final data = <String, dynamic>{
      'listen_addr': '0.0.0.0:18789',
      'org': 'org-1',
      'features': ['chat', 'skills'],
    };

    await service.writeConfig(data);
    final cfg = await service.readConfig();

    expect(cfg['listen_addr'], '0.0.0.0:18789');
    expect(cfg['org'], 'org-1');
    expect((cfg['features'] as List).length, 2);
  });

  test('saveEnvValue and getEnvValue round trip and update', () async {
    await service.saveEnvValue('API_KEY', 'first');
    expect(await service.getEnvValue('API_KEY'), 'first');

    await service.saveEnvValue('API_KEY', 'updated');
    expect(await service.getEnvValue('API_KEY'), 'updated');

    await service.saveEnvValue('BASE_URL', 'http://localhost:18789');
    expect(await service.getEnvValue('BASE_URL'), 'http://localhost:18789');
  });

  test(
    'getEnvValue returns null when env file missing or key absent',
    () async {
      expect(await service.getEnvValue('MISSING_KEY'), isNull);

      await service.saveEnvValue('ONLY_KEY', 'value');
      expect(await service.getEnvValue('ANOTHER_KEY'), isNull);
    },
  );

  test('getSystemInfo returns expected fields', () async {
    final info = await service.getSystemInfo();

    expect(info['os'], isA<String>());
    expect(info['os_version'], isA<String>());
    expect(info['dart_version'], isA<String>());
    expect(info['hostname'], isA<String>());
    expect(info['cpus'], isA<int>());
  });

  test('isServiceRunning detects open port 18789', () async {
    ServerSocket? server;
    try {
      server = await ServerSocket.bind(
        InternetAddress.loopbackIPv4,
        18789,
        shared: true,
      );
      expect(await service.isServiceRunning(), isTrue);
    } on SocketException {
      // If something else in the environment already occupies the port,
      // the service should still report "running".
      expect(await service.isServiceRunning(), isTrue);
    } finally {
      await server?.close();
    }
  });




}
