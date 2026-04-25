import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/models/dashboard.dart';

class MockAuthService implements AuthService {
  @override
  String get baseUrl => 'http://mock';
  @override
  Future<AuthUser> login(String email, String password) async => const AuthUser(id: '1', email: 'test', name: 'test', role: 'admin', organizationId: 'org1', token: 'mock_token');
  @override
  Future<void> logout(String token) async {}
  @override
  Future<bool> get isAuthenticated async => true;
  @override
  Future<String?> get token async => 'mock_token';
  @override
  Future<String?> get tenantId async => 'mock_tenant';
  @override
  void dispose() {}
}

class MockApiService implements ApiService {
  @override
  Future<DashboardSnapshot> getDashboard() async {
    return DashboardSnapshot(
      organization: const Organization(
        tier: 'Free',
        id: 'org1',
        name: 'Test Org',
        roleProfiles: [],
        members: [],
        domain: 'test.com',
      ),
      agents: [],
      statuses: [],
      meetings: [],
      costs: const CostSummary(totalCostUSD: 0.0, totalTokens: 0, totalActions: 0, agents: []),
      updatedAt: DateTime.now(),
    );
  }

  @override
  Future<void> scaleAgents(String role, int count) async {}
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

void main() {
  testWidgets('CUJ: Verify Welcome Checklist appears on Dashboard', (WidgetTester tester) async {
    final mockAuth = MockAuthService();
    final mockApi = MockApiService();


    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authServiceProvider.overrideWithValue(mockAuth),
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: Consumer(
          builder: (context, ref, child) {
            final router = ref.watch(routerProvider);
            // Wait until the end of the frame, then navigate
            WidgetsBinding.instance.addPostFrameCallback((_) {
               if (router.routerDelegate.currentConfiguration.uri.toString() != '/dashboard') {
                 router.go('/dashboard');
               }
            });
            return MaterialApp.router(
              routerConfig: router,
            );
          },
        ),
      ),
    );


    // We don't pumpAndSettle because there might be infinite animations on the dashboard.
    for (int i = 0; i < 10; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }


    // Verify Welcome Checklist is displayed
    expect(find.text('Welcome Checklist'), findsOneWidget);
    expect(find.text('Business live'), findsOneWidget);
    expect(find.text('Add 3 more products'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);
    expect(find.text('Share your link with a friend'), findsOneWidget);

    // Test tapping a checklist item
    await tester.tap(find.text('Share your link with a friend'));

    // We don't pumpAndSettle because there might be infinite animations on the dashboard.
    for (int i = 0; i < 10; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }

    expect(find.text('Link copied to clipboard!'), findsOneWidget);
  });
}
