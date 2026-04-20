// CUJ: Dashboard Screen (Additional)
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';

DashboardSnapshot _snap(String orgName) => DashboardSnapshot(
  organization: Organization(id: 'o1', name: orgName, domain: 'd.com', members: const [], roleProfiles: const []),
  meetings: const [],
  costs: CostSummary(totalCostUSD: 0, totalTokens: 0, agents: const []),
  agents: const [],
  statuses: const [],
  updatedAt: DateTime(2025),
);

Widget _wrap(String orgName) => ProviderScope(
  overrides: [dashboardProvider.overrideWith((ref) async => _snap(orgName))],
  child: const MaterialApp(home: DashboardScreen()),
);

void main() {
  group('CUJ: Dashboard Screen Additional', () {
    testWidgets('renders Scaffold', (t) async { await t.pumpWidget(_wrap('Org1')); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('renders AppBar', (t) async { await t.pumpWidget(_wrap('Org1')); await t.pumpAndSettle(); expect(find.byType(AppBar), findsAtLeastNWidgets(1)); });
    testWidgets('org name renders', (t) async { await t.pumpWidget(_wrap('Test Corp')); await t.pumpAndSettle(); expect(find.textContaining('Test Corp'), findsOneWidget); });
    testWidgets('narrow viewport', (t) async {
      t.view.physicalSize = const Size(360, 640); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap('Org1')); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('wide viewport', (t) async {
      t.view.physicalSize = const Size(1280, 800); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap('Org1')); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('multiple pumps', (t) async {
      await t.pumpWidget(_wrap('Org1')); await t.pump(); await t.pump(const Duration(milliseconds: 100)); expect(find.byType(Scaffold), findsOneWidget);
    });
    testWidgets('ProviderScope present', (t) async { await t.pumpWidget(_wrap('Org1')); await t.pump(); expect(find.byType(ProviderScope), findsOneWidget); });
    testWidgets('pumpAndSettle no crash', (t) async { await t.pumpWidget(_wrap('Org1')); await t.pumpAndSettle(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('rebuild no crash', (t) async { await t.pumpWidget(_wrap('Org1')); await t.pump(); await t.pumpWidget(_wrap('Org2')); await t.pump(); expect(find.byType(Scaffold), findsOneWidget); });
    testWidgets('medium viewport', (t) async {
      t.view.physicalSize = const Size(768, 1024); t.view.devicePixelRatio = 1.0;
      addTearDown(t.view.resetPhysicalSize); addTearDown(t.view.resetDevicePixelRatio);
      await t.pumpWidget(_wrap('Org1')); await t.pump(); expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
