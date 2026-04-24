import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/landing_screen.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

Widget _wrapApp(http.Client mockClient) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => const LandingScreen()),
      GoRoute(path: '/business_setup', builder: (context, state) => const BusinessSetupWizardScreen()),
      GoRoute(path: '/dashboard', builder: (context, state) => const Scaffold(body: Text('Dashboard'))),
    ],
  );

  return ProviderScope(
    overrides: [
      apiServiceProvider.overrideWithValue(
        ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient),
      ),
      clientSettingsProvider.overrideWith(
        (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
          ClientSettings(backendUrl: 'http://localhost', standaloneMode: false),
        ),
      ),
      backendUrlProvider.overrideWithValue('http://localhost'),
    ],
    child: MaterialApp.router(
      routerConfig: router,
    ),
  );
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  testWidgets('Business Setup Wizard End-to-End Flow', (tester) async {
    tester.view.physicalSize = const Size(1080, 1920);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    final mockClient = MockHttpClient();

    // Mock the configure endpoint
    when(() => mockClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')))
        .thenAnswer((_) async => http.Response('{"status": "ok"}', 200));

    await tester.pumpWidget(_wrapApp(mockClient));
    await tester.pumpAndSettle();

    // 1. Start from Landing Page and navigate to Business Setup
    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Start Business Setup'));
    await tester.pumpAndSettle();
    await tester.tap(find.widgetWithText(ElevatedButton, 'Start Business Setup'));
    await tester.pumpAndSettle();

    // Verify we are on the first step of the wizard
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    // Navigate to Step 1
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Business Type
    expect(find.text('What kind of business are you building?'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Name & Description
    await tester.enterText(find.byType(TextField).first, 'My Awesome Store');
    await tester.enterText(find.byType(TextField).last, 'Selling the best widgets');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: What do you sell
    expect(find.text('What do you sell?'), findsOneWidget);
    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payments
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Admin Details
    await tester.enterText(find.byType(TextField).at(0), 'Alice Admin');
    await tester.enterText(find.byType(TextField).at(1), 'alice@example.com');
    await tester.enterText(find.byType(TextField).at(2), 'securepassword');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 6: Review & Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Launch My Business →'), findsOneWidget);

    // Launch!
    await tester.tap(find.text('Launch My Business →'));
    await tester.pump();
    await tester.pump(const Duration(seconds: 1));
    await tester.pumpAndSettle();

    // API bypassed because auth is null

    // Verify it navigates to Dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });
}
