// CUJ: Cost Dashboard Screen
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/cost_dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class _SeededCostApiService extends ApiService {
  _SeededCostApiService() : super(baseUrl: 'http://test-host', token: 'tok');
  @override
  Future<Map<String, dynamic>> getDashboard() async => {
    'organization': {'id': 'org-1', 'name': 'Acme', 'domain': 'acme.com', 'members': [], 'role_profiles': []},
    'costs': {'total_cost_usd': 10.0, 'total_tokens': 5000, 'agents': []},
    'agents': [],
    'meetings': [],
    'statuses': [],
    'updated_at': DateTime(2025).toIso8601String(),
  };
}

Widget _wrap() => ProviderScope(
  overrides: [apiServiceProvider.overrideWithValue(_SeededCostApiService())],
  child: const MaterialApp(home: CostDashboardScreen()),
);

void main() {
  group('CUJ: Cost Dashboard', () {
    testWidgets('renders Scaffold', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('renders AppBar', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(AppBar), findsAtLeastNWidgets(1)); });
    testWidgets('narrow viewport no overflow', (t) async {
      t.view.physicalSize = const Size(360, 640); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('wide viewport no overflow', (t) async {
      t.view.physicalSize = const Size(1280, 800); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('multiple pumps no crash', (t) async {
      await t.pumpWidget(_wrap()); await t.pump(); await t.pump(const Duration(milliseconds: 100)); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('ProviderScope present', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(ProviderScope), findsOneWidget); });
    testWidgets('no crash on pumpAndSettle', (t) async { await t.pumpWidget(_wrap()); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('rebuild no crash', (t) async { await t.pumpWidget(_wrap()); await t.pump(); await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('MaterialApp present', (t) async { await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(MaterialApp), findsOneWidget); });
    testWidgets('medium viewport no overflow', (t) async {
      t.view.physicalSize = const Size(768, 1024); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap()); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
