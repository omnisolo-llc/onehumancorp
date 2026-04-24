import 'dart:ui';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/hybrid_observability_widget.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/agent.dart';

void main() {
  testWidgets('HybridObservabilityWidget renders data from dashboardProvider', (WidgetTester tester) async {
    final mockSnapshot = DashboardSnapshot(
      organization: const Organization(
        id: 'org-1',
        name: 'Test Org',
        domain: 'test.com',
        members: [],
        roleProfiles: [],
      ),
      meetings: [],
      costs: const CostSummary(totalCostUSD: 100, totalTokens: 1000, agents: []),
      agents: [
        Agent(id: 'a1', name: 'Agent 1', role: 'engineer', status: 'running', organizationId: 'org-1', createdAt: DateTime.now()),
        Agent(id: 'a2', name: 'Agent 2', role: 'manager', status: 'idle', organizationId: 'org-1', createdAt: DateTime.now()),
      ],
      statuses: [
        const StatusBucket(status: 'pending', count: 5),
        const StatusBucket(status: 'completed', count: 10),
      ],
      updatedAt: DateTime.now(),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) async => mockSnapshot),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridObservabilityWidget(),
          ),
        ),
      ),
    );

    await tester.pump(); // Start loading data

    expect(find.text('Hybrid Swarm Observability'), findsOneWidget);
    expect(find.text('Active Agents'), findsOneWidget);
    expect(find.text('Total Tasks'), findsOneWidget);
    
    // Active agents count (1 running)
    expect(find.text('1'), findsOneWidget);
    
    // Total tasks count (5 + 10 = 15)
    expect(find.text('15'), findsOneWidget);
  });

  testWidgets('HybridObservabilityWidget renders loading state', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) => Completer<DashboardSnapshot>().future),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridObservabilityWidget(),
          ),
        ),
      ),
    );

    expect(find.byType(CircularProgressIndicator), findsOneWidget);
  });

  testWidgets('HybridObservabilityWidget renders error state', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) async => throw Exception('Fail')),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridObservabilityWidget(),
          ),
        ),
      ),
    );

    await tester.pump(); // Ensure error state is rendered

    final texts = tester.widgetList<Text>(find.byType(Text));
    for (final t in texts) {
      print('DEBUG Text found: ${t.data}');
    }

    expect(find.textContaining('Fail'), findsOneWidget);
  });
}
