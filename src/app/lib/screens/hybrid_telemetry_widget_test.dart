import 'dart:ui';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/hybrid_telemetry_widget.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/widgets/shimmer_loading.dart';

void main() {
  testWidgets('HybridTelemetryWidget renders data when loaded', (WidgetTester tester) async {
    final mockSnapshot = DashboardSnapshot(
      organization: const Organization(id: 'org-1', name: 'Test Org', domain: 'test.com', members: [], roleProfiles: []),
      meetings: [],
      costs: const CostSummary(totalCostUSD: 100, totalTokens: 1000, agents: []),
      agents: [],
      statuses: [],
      updatedAt: DateTime.now(),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) async => mockSnapshot),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridTelemetryWidget(),
          ),
        ),
      ),
    );

    await tester.pump(); // Start loading data

    expect(find.text('Hybrid Deployment Telemetry'), findsOneWidget);
    expect(find.textContaining('Cloud throughput: high'), findsOneWidget);
    expect(find.textContaining('Local throughput: stable'), findsOneWidget);
  });

  testWidgets('HybridTelemetryWidget renders loading state', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) => Completer<DashboardSnapshot>().future),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridTelemetryWidget(),
          ),
        ),
      ),
    );

    expect(find.byType(ShimmerLoading), findsOneWidget);
  });

  testWidgets('HybridTelemetryWidget renders error state', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWith((ref) async => throw Exception('Fail')),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridTelemetryWidget(),
          ),
        ),
      ),
    );

    await tester.pump();

    expect(find.textContaining('Fail'), findsOneWidget);
  });
}
