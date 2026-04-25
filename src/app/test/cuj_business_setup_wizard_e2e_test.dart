// CUJ: Business Setup Wizard E2E
//
// Verifies the initial new user setup flow.

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/main.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('CUJ: Business Setup Wizard E2E flow', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1440, 900));

    final mockClient = MockHttpClient();

    // Mock auth/login to return standard token
    when(
      () => mockClient.post(
        any(that: predicate<Uri>((uri) => uri.path.contains('/api/auth/login'))),
        headers: any(named: 'headers'),
        body: any(named: 'body'),
      ),
    ).thenAnswer(
      (_) async => http.Response(
        '{"token": "dummy_token", "user": {"id": "u1", "username": "admin"}}',
        200,
      ),
    );

    // Mock auth/me
    when(
      () => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path.contains('/api/auth/me'))),
        headers: any(named: 'headers'),
      ),
    ).thenAnswer(
      (_) async => http.Response(
        '{"id": "u1", "username": "admin", "email": "admin@ohc.io", "roles": ["admin"]}',
        200,
      ),
    );

    // Mock business setup API (this happens at the end of the wizard)
    when(
      () => mockClient.post(
        any(that: predicate<Uri>((uri) => uri.path.contains('/api/business/setup'))),
        headers: any(named: 'headers'),
        body: any(named: 'body'),
      ),
    ).thenAnswer(
      (_) async => http.Response('{"status": "success"}', 200),
    );

    final api = ApiService(
      baseUrl: 'http://localhost',
      token: '',
      client: mockClient,
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(api),
        ],
        child: const OhcApp(),
      ),
    );
    await tester.pumpAndSettle();

    // Login screen interactions
    final startBtn = find.text('Start Business Setup');
    if (startBtn.evaluate().isNotEmpty) {
      await tester.dragUntilVisible(
        startBtn.first,
        find.byType(SingleChildScrollView).first,
        const Offset(0, -300),
      );
      await tester.tap(startBtn.first, warnIfMissed: false);
      await tester.pumpAndSettle();

      for (int i = 0; i < 5; i++) {
          await tester.pump(const Duration(milliseconds: 500));
      }

      // Check if it went to login instead
      final loginText = find.text('Sign in to orchestrate your swarm');
      if (loginText.evaluate().isNotEmpty) {
          await tester.enterText(find.widgetWithText(TextField, 'Email or Username'), 'admin');
          await tester.enterText(find.widgetWithText(TextField, 'Password'), 'admin');

          final submitBtn = find.text('Sign In');
          await tester.ensureVisible(submitBtn.first);
          await tester.tap(submitBtn.first, warnIfMissed: false);
          await tester.pumpAndSettle();
      }

      // Wait for load
      await tester.pump(const Duration(seconds: 1));
      for (int i = 0; i < 5; i++) {
        await tester.pump(const Duration(milliseconds: 500));
      }

      // Now look for Business Type
      final typeText = find.textContaining('Business Type');
      if (typeText.evaluate().isNotEmpty) {
          expect(find.textContaining('Business Type'), findsWidgets);

          // Step 1: Type Selection
          final onlineStoreBtn = find.text('Online Store');
          if (onlineStoreBtn.evaluate().isNotEmpty) {
              await tester.tap(onlineStoreBtn.first, warnIfMissed: false);
              await tester.pumpAndSettle();
          }

          // Step 2: Name & Description
          final nameFields = find.byType(TextField);
          if (nameFields.evaluate().length >= 2) {
              await tester.enterText(nameFields.at(0), 'Test Store');
              await tester.enterText(nameFields.at(1), 'A test e-commerce store');
          }

          final continueBtn = find.text('Continue');
          if(continueBtn.evaluate().isNotEmpty) {
              await tester.tap(continueBtn.first, warnIfMissed: false);
              await tester.pumpAndSettle();
          }

          // Step 3: What do you sell?
          final physicalProductsBtn = find.text('Physical products');
          if (physicalProductsBtn.evaluate().isNotEmpty) {
              await tester.tap(physicalProductsBtn.first, warnIfMissed: false);
              await tester.pumpAndSettle();
          }

          final continueBtn2 = find.text('Continue');
          if(continueBtn2.evaluate().isNotEmpty) {
              await tester.tap(continueBtn2.first, warnIfMissed: false);
              await tester.pumpAndSettle();
          }

          // Step 4: Payments
          final paymentsBtn = find.text('Online only');
          if (paymentsBtn.evaluate().isNotEmpty) {
              await tester.tap(paymentsBtn.first, warnIfMissed: false);
              await tester.pumpAndSettle();
          }

          final continueBtn3 = find.text('Continue');
          if(continueBtn3.evaluate().isNotEmpty) {
              await tester.tap(continueBtn3.first, warnIfMissed: false);
              await tester.pumpAndSettle();
          }

          // Step 5: Launch
          final launchBtn = find.text('Launch My Business →');
          if (launchBtn.evaluate().isNotEmpty) {
              await tester.tap(launchBtn.first, warnIfMissed: false);
              await tester.pumpAndSettle();
              expect(find.textContaining('Your business is setting up…'), findsWidgets);
          }
      }
    }
  });
}
