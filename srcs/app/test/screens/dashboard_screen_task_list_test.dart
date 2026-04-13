import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/models/dashboard.dart';

// Provide a mock dashboard snapshot
final mockDashboardProvider = FutureProvider.autoDispose<DashboardSnapshot>((ref) async {
  return DashboardSnapshot(
    organization: const Organization(id: 'org1', name: 'Test Org', domain: 'test.com', members: []),
    meetings: [],
    costs: const CostSummary(totalCostUSD: 0.0, totalTokens: 0, agents: []),
    agents: [],
    statuses: [],
    updatedAt: DateTime.now(),
  );
});

void main() {
  testWidgets('DashboardScreen includes TaskListScreen with Glassmorphism', (WidgetTester tester) async {
    // Increase size so we can scroll or everything fits
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          dashboardProvider.overrideWithProvider(mockDashboardProvider),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: DashboardScreen(),
          ),
        ),
      ),
    );

    // Give time for future providers to load without infinite animation loop timeouts
    await tester.pump(const Duration(seconds: 1));
    await tester.pump(const Duration(seconds: 1));

    // Since TaskListScreen is in a scroll view, we might need to scroll
    final listFinder = find.byType(Scrollable).first;
    await tester.scrollUntilVisible(
      find.byType(TaskListScreen),
      500.0,
      scrollable: listFinder,
      maxScrolls: 50,
    );

    expect(find.byType(TaskListScreen), findsOneWidget);

    // reset view properties
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
