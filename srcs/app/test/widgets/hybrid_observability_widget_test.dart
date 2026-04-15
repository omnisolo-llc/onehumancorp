import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/hybrid_observability_widget.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/agent.dart';

void main() {
  testWidgets('HybridObservabilityWidget renders correctly with OHC tokens', (WidgetTester tester) async {
    final mockSnapshot = DashboardSnapshot(
      organization: const Organization(id: '1', name: 'Test Org', domain: 'test.com', members: []),
      meetings: [],
      costs: const CostSummary(totalCostUSD: 0, totalTokens: 0, agents: []),
      agents: [
        Agent(
          id: '1',
          name: 'Agent 1',
          role: 'Analyst',
          status: 'running',
          organizationId: '1',
          createdAt: DateTime.now(),
        ),
      ],
      statuses: [
        const StatusBucket(status: 'Running', count: 5),
      ],
      updatedAt: DateTime.now(),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) => mockSnapshot),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridObservabilityWidget(),
          ),
        ),
      ),
    );

    expect(find.text('Hybrid Swarm Observability'), findsOneWidget);
    expect(find.text('Active Agents'), findsOneWidget);
    expect(find.text('1'), findsOneWidget); // 1 active agent
    expect(find.text('5'), findsOneWidget); // 5 total tasks

    final backdropFilter = tester.widget<BackdropFilter>(find.byType(BackdropFilter).first);
    final imageFilter = backdropFilter.filter as ImageFilter;

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, Colors.white.withOpacity(0.03));
  });
}
