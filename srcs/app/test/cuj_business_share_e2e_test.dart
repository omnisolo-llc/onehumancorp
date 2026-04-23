import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

final testOrganization = Organization(
  id: 'org1',
  name: 'Test Business',
  domain: 'test.ohc.io',
  members: const [],
  roleProfiles: const [],
);

final testDashboardData = DashboardSnapshot(
  organization: testOrganization,
  meetings: const [],
  costs: const CostSummary(totalCostUSD: 0, totalTokens: 0, agents: []),
  agents: const [],
  statuses: const [],
  updatedAt: DateTime.now(),
);

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  Widget createTestApp(ApiService apiService, AuthState authState) {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(apiService),
        authStateProvider.overrideWithValue(AsyncValue.data(authState)),
      ],
      child: Consumer(
        builder: (context, ref, _) {
          final router = ref.watch(routerProvider);
          return MaterialApp.router(
            routerConfig: router,
          );
        },
      ),
    );
  }

  testWidgets('E2E: CUJ Business Share and Viral Storefront', (tester) async {
    final mockClient = MockHttpClient();
    when(() => mockClient.get(any(), headers: any(named: 'headers')))
        .thenAnswer((_) async => http.Response(jsonEncode(testDashboardData.toJson()), 200));

    final apiService = ApiService(baseUrl: 'http://test', token: 'token', client: mockClient);
    final authState = AuthState(userId: 'u1', token: 'token', role: 'admin');

    await tester.pumpWidget(createTestApp(apiService, authState));
    await tester.pumpAndSettle();

    // App should start at /dashboard due to login redirect and valid authState
    // However, if we start at login, we might need to manually trigger it, but
    // the GoRouter config in router.dart automatically handles redirects.

    // Force navigation to dashboard just in case the redirect logic needs a push in tests
    final BuildContext context = tester.element(find.byType(Router<Object>));
    context.go('/dashboard');
    await tester.pumpAndSettle();

    // Verify BusinessShareWidget is visible on the Dashboard
    expect(find.text('Share My Business'), findsOneWidget);
    expect(find.text('Test Business'), findsOneWidget); // Organization name should be rendered

    // Tap the "Copy Link" button
    final copyLinkButton = find.text('Copy Link');
    expect(copyLinkButton, findsOneWidget);

    await tester.tap(copyLinkButton);
    await tester.pumpAndSettle(const Duration(milliseconds: 500));

    // Check if SnackBar appears confirming copy
    expect(find.textContaining('Link copied for Clipboard'), findsOneWidget);

    // Wait for the SnackBar to dismiss before navigating to a new route.
    await tester.pumpAndSettle(const Duration(seconds: 4));

    // Navigate to Storefront
    context.go('/storefront');
    await tester.pumpAndSettle();

    // Verify Storefront Screen elements
    expect(find.text('Welcome to Our Store'), findsOneWidget);

    // Check for Viral Footer
    final viralFooter = find.text('Built with OHC — Start your free business →');
    expect(viralFooter, findsOneWidget);

    // Tap the Viral Footer
    await tester.tap(viralFooter);
    await tester.pumpAndSettle();

    // Verify Navigation to Landing Screen
    expect(find.text('Landing Screen'), findsNothing); // Just ensuring it navigates, typically would check for landing elements
    // The landing screen has "Start Business Setup"
    expect(find.text('Start Business Setup'), findsOneWidget);
  });
}

extension DashboardSnapshotToJson on DashboardSnapshot {
  Map<String, dynamic> toJson() {
    return {
      'organization': {
        'id': organization.id,
        'name': organization.name,
        'domain': organization.domain,
        'members': [],
        'roleProfiles': [],
      },
      'meetings': [],
      'costs': {
        'totalCostUSD': 0,
        'totalTokens': 0,
        'agents': [],
      },
      'agents': [],
      'statuses': [],
      'updatedAt': updatedAt.toIso8601String(),
    };
  }
}