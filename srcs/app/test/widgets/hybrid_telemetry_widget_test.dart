import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/hybrid_telemetry_widget.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  testWidgets('HybridTelemetryWidget displays correct data', (WidgetTester tester) async {
    final mockApi = MockApiService();

    when(() => mockApi.getDashboard()).thenAnswer(
      (_) async => DashboardSnapshot(
        organization: Organization(id: '1', name: 'Test Org', domain: 'test.com', members: [], roleProfiles: []),
        meetings: [],
        costs: CostSummary(totalCostUSD: 0.0, totalTokens: 0, agents: []),
        agents: [],
        statuses: [
          StatusBucket(status: 'pending', count: 8),
          StatusBucket(status: 'running', count: 2),
        ],
        updatedAt: DateTime.now(),
      ),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: HybridTelemetryWidget(),
          ),
        ),
      ),
    );

    // Initial loading state
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Wait for the FutureProvider to resolve
    await tester.pumpAndSettle();

    expect(find.text('Hybrid Deployment Telemetry'), findsOneWidget);
    expect(find.text('Cloud-Native Throughput'), findsOneWidget);
    expect(find.text('Standalone Throughput'), findsOneWidget);
    expect(find.text('5 tasks'), findsNWidgets(2)); // 10 / 2 = 5 cloud, 5 standalone
  });
}
