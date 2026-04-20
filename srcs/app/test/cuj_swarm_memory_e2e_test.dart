// CUJ: Swarm Memory Screen
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/swarm_memory_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class _SeededSwarmApi extends ApiService {
  _SeededSwarmApi() : super(baseUrl: 'http://test-host', token: 'tok');
  @override
  Future<Map<String, dynamic>> getSwarmMemory() async => {'nodes': [], 'edges': []};
}

Widget _wrap() => ProviderScope(
  overrides: [apiServiceProvider.overrideWithValue(_SeededSwarmApi())],
  child: const MaterialApp(home: SwarmMemoryScreen()),
);

void main() {
  group('CUJ: Swarm Memory Screen', () {
    testWidgets('renders Scaffold', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('renders AppBar', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(AppBar), findsAtLeastNWidgets(1)); });
    testWidgets('narrow viewport', (t) async {
      t.view.physicalSize = const Size(360, 640); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('wide viewport', (t) async {
      t.view.physicalSize = const Size(1280, 800); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('multiple pumps', (t) async {
      await t.pumpWidget(_wrap()); await t.pump(); await t.pump(const Duration(milliseconds: 100)); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('ProviderScope present', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(ProviderScope), findsOneWidget); });
    testWidgets('pumpAndSettle no crash', (t) async { await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('rebuild no crash', (t) async { await t.pumpWidget(_wrap()); await t.pump(); await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('MaterialApp present', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(MaterialApp), findsOneWidget); });
    testWidgets('medium viewport', (t) async {
      t.view.physicalSize = const Size(768, 1024); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
