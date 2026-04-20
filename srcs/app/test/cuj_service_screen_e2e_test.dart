// CUJ: Service Screen
// ServiceScreen uses localManagerServiceProvider (not apiServiceProvider).
// We override localManagerServiceProvider with a no-op stub.
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/service_screen.dart';
import 'package:ohc_app/services/local_manager_service.dart';

/// Stub that never touches the network or filesystem.
class _StubLocalManagerService extends LocalManagerService {
  _StubLocalManagerService() : super(homeOverride: '/tmp/ohc-test-stub');
  @override
  Future<bool> isServiceRunning() async => false;
  @override
  Future<void> startService() async {}
  @override
  Future<void> stopService() async {}
  @override
  Future<void> restartService() async {}
  @override
  Future<String> runDoctor() async => 'OK';
  @override
  Future<Map<String, dynamic>> getSystemInfo() async => {'os': 'test', 'status': 'ok'};
  @override
  Future<Map<String, dynamic>> readConfig() async => {};
  @override
  Future<void> writeConfig(Map<String, dynamic> config) async {}
}

Widget _wrap() => ProviderScope(
  overrides: [
    localManagerServiceProvider.overrideWithValue(_StubLocalManagerService()),
  ],
  child: const MaterialApp(home: ServiceScreen()),
);

void main() {
  group('CUJ: Service Screen', () {
    testWidgets('renders Scaffold', (t) async {
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('renders AppBar', (t) async {
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(AppBar), findsAtLeastNWidgets(1));
    });
    testWidgets('narrow viewport', (t) async {
      t.view.physicalSize = const Size(800, 640); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('wide viewport', (t) async {
      t.view.physicalSize = const Size(1280, 800); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('multiple pumps', (t) async {
      await t.pumpWidget(_wrap()); await t.pump();
      await t.pump(const Duration(milliseconds: 100));
      expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('ProviderScope present', (t) async {
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(ProviderScope), findsOneWidget);
    });
    testWidgets('rebuild no crash', (t) async {
      await t.pumpWidget(_wrap()); await t.pump();
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('MaterialApp present', (t) async {
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(MaterialApp), findsOneWidget);
    });
    testWidgets('medium viewport', (t) async {
      t.view.physicalSize = const Size(1024, 1024); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump();
      expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('service screen does not crash on startup', (t) async {
      await t.pumpWidget(_wrap());
      await t.pump(const Duration(milliseconds: 100));
      expect(find.byType(MaterialApp), findsOneWidget);
    });
  });
}
