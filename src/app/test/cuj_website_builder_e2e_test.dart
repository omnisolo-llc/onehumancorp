import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/website_builder_onboarding.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/screens/login_screen.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:ohc_app/services/api_service.dart';

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

class MockAuthNotifier extends AuthNotifier {
  @override
  Future<AuthUser?> build() async {
    return const AuthUser(
      id: 'u1',
      email: 'user@example.com',
      name: 'Test User',
      role: 'admin',
      organizationId: 'org-1',
      token: 'fake-jwt',
    );
  }
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  testWidgets('Website Builder Onboarding Wizard flow from Home', (WidgetTester tester) async {
    final mockClient = MockHttpClient();
    final apiService = ApiService(baseUrl: 'http://localhost', token: 'mock-token', client: mockClient);

    when(() => mockClient.get(any(), headers: any(named: 'headers')))
        .thenAnswer((_) async => http.Response(
            jsonEncode({
              'organization': {
                'id': '1',
                'name': 'Test Org',
                'domain': 'test.com',
                'members': [],
                'roleProfiles': []
              },
              'agents': [],
              'meetings': [],
              'statuses': [],
              'costSummary': {
                'totalCostUSD': 0.0,
                'totalTokens': 0,
                'agents': []
              },
              'updatedAt': DateTime.now().toIso8601String()
            }),
            200));

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const LoginScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const DashboardScreen(),
        ),
        GoRoute(
          path: '/wizards/website-builder',
          builder: (context, state) => const WebsiteBuilderOnboardingScreen(),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authStateProvider.overrideWith(() => MockAuthNotifier()),
          backendUrlProvider.overrideWithValue('http://localhost'),
          apiServiceProvider.overrideWithValue(apiService),
        ],
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );
    await tester.pumpAndSettle();

    // Start from home/dashboard by bypassing login
    router.go('/dashboard');
    await tester.pumpAndSettle();

    // We are on dashboard, tap 'Build My Website'
    expect(find.text('Build My Website'), findsOneWidget);
    await tester.tap(find.text('Build My Website'));
    await tester.pumpAndSettle();

    // Step 1
    expect(find.text('Select a Template'), findsOneWidget);
    await tester.tap(find.text('Minimal'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2
    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    await tester.tap(find.text('Ocean'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Generate a logo for me'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3
    expect(find.text('Add Product or Service'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Test Product');
    await tester.enterText(find.byType(TextField).at(1), '100');
    await tester.enterText(find.byType(TextField).at(2), 'Test Description');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4
    expect(find.text('Connect a Domain'), findsOneWidget);
    await tester.tap(find.text('Own Domain'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5
    expect(find.text('Go Live'), findsOneWidget);

    when(() => mockClient.post(
      any(),
      headers: any(named: 'headers'),
      body: any(named: 'body'),
    )).thenAnswer((_) async => http.Response('{}', 200));

    await tester.tap(find.text('Publish'));
    await tester.pumpAndSettle();

    // Assert we are routed back to Dashboard
    expect(find.text('Dashboard'), findsWidgets);
  });
}
