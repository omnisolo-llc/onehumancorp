import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/dashboard.dart';

// Provide a mock dashboard snapshot
final mockDashboardProvider = FutureProvider.autoDispose<DashboardSnapshot>((ref) async {
  return DashboardSnapshot(
    organization: const Organization(id: 'org1', name: 'Test Org', domain: 'test.com', members: [], roleProfiles: [], onboardingStatus: OnboardingStatus()),
    meetings: [],
    costs: const CostSummary(totalCostUSD: 0.0, totalTokens: 0, agents: []),
    agents: [],
    statuses: [],
    updatedAt: DateTime.now(),
  );
});

void main() {
  testWidgets('DashboardScreen renders Welcome Checklist with correct items', (WidgetTester tester) async {
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

    // Wait for FutureProvider to load
    await tester.pump(const Duration(seconds: 1));
    await tester.pump(const Duration(seconds: 1));

    expect(find.text('Welcome Checklist'), findsOneWidget);
    expect(find.text("You're set up! Here's what to do next:"), findsOneWidget);
    expect(find.text('✅ Business live'), findsOneWidget);
    expect(find.text('⬜ Add 3 more products'), findsOneWidget);
    expect(find.text('⬜ Connect Instagram'), findsOneWidget);
    expect(find.text('⬜ Share your link with a friend'), findsOneWidget);

    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
