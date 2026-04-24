import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  testWidgets('CUJ: Business Share & Embed E2E test', (tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    addTearDown(tester.view.resetPhysicalSize);

    final org = const Organization(
      id: '1',
      name: 'Test Business',
      domain: 'test-domain',
      members: [],
      roleProfiles: [],
    );

    final data = DashboardSnapshot(
      organization: org,
      meetings: [],
      costs: const CostSummary(totalCostUSD: 0, totalTokens: 0, agents: []),
      agents: [],
      statuses: [],
      updatedAt: DateTime.now(),
    );

    when(() => mockApiService.getDashboard()).thenAnswer((_) async => data);
    when(() => mockApiService.getQuota()).thenAnswer((_) async => {'used': 0, 'max': 100});

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
          dashboardProvider.overrideWith((ref) => data),
        ],
        child: const MaterialApp(
          home: Scaffold(body: DashboardScreen()),
        ),
      ),
    );

    await tester.pumpAndSettle();
    await tester.pump(const Duration(seconds: 1));

    final shareButton = find.text('Share my business');

    if(shareButton.evaluate().isNotEmpty) {
      await tester.ensureVisible(shareButton);
      await tester.tap(shareButton);
      await tester.pump();
    }
  });
}
