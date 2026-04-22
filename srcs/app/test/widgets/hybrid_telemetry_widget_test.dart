import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/hybrid_telemetry_widget.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  testWidgets('HybridTelemetryWidget displays correct data', (WidgetTester tester) async {
    final mockApi = MockApiService();

    when(() => mockApi.getDashboard()).thenAnswer(
      (_) async => const DashboardSnapshot(
        totalTasks: 10,
        runningTasks: 2,
        pendingTasks: 8,
        completedTasks: 0,
        agentCount: 5,
        budgetBurnRate: 0.0,
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
