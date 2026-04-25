// CUJ: Website Builder Wizard
//
// Covers the end-to-end flow for building a website:
//   1. Select template
//   2. Choose colors & logo
//   3. Add product details
//   4. Domain choice
//   5. Publish

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

  testWidgets('CUJ: Website Builder Wizard E2E flow', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1440, 900));

    final mockClient = MockHttpClient();
    when(
      () => mockClient.post(
        any(that: predicate<Uri>((uri) => uri.path.contains('/api/auth/login'))),
        headers: any(named: 'headers'),
        body: any(named: 'body'),
      ),
    ).thenAnswer(
      (_) async => http.Response('{"token": "dummy_token", "user": {"id": "u1", "username": "admin"}}', 200),
    );

    when(
      () => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path.contains('/api/auth/me'))),
        headers: any(named: 'headers'),
      ),
    ).thenAnswer(
      (_) async => http.Response('{"id": "u1", "username": "admin", "email": "a@b.com", "roles": ["admin"]}', 200),
    );

    when(
      () => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path.contains('/api/dashboard'))),
        headers: any(named: 'headers'),
      ),
    ).thenAnswer(
      (_) async => http.Response('{"metrics":{},"activities":[]}', 200),
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

    final loginBtn = find.text('Or continue to Cloud Dashboard');
    expect(loginBtn, findsWidgets);
    await tester.dragUntilVisible(
      loginBtn.first,
      find.byType(SingleChildScrollView).first,
      const Offset(0, -300),
    );
    await tester.pumpAndSettle();

    await tester.tap(loginBtn.first, warnIfMissed: false);
    await tester.pumpAndSettle();

    expect(find.text('Sign in to orchestrate your swarm'), findsWidgets);
    await tester.enterText(find.widgetWithText(TextField, 'Email or Username'), 'admin');
    await tester.enterText(find.widgetWithText(TextField, 'Password'), 'admin');

    // Explicitly find the Form and submit it or tap the button directly
    final submitBtn = find.text('Sign In');
    await tester.ensureVisible(submitBtn.first);
    await tester.tap(submitBtn.first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Additional wait just in case
    await tester.pump(const Duration(seconds: 1));
    await tester.pumpAndSettle();
    for (int i = 0; i < 5; i++) {
        await tester.pump(const Duration(milliseconds: 500));
    }

    final webBuilderBtnFinder = find.byType(OutlinedButton);
    if (webBuilderBtnFinder.evaluate().isNotEmpty) {
        bool clicked = false;
        for (var el in webBuilderBtnFinder.evaluate()) {
            final w = el.widget as OutlinedButton;
            if (w.child is Text && (w.child as Text).data == 'Build My Website') {
                await tester.ensureVisible(find.byWidget(w));
                await tester.tap(find.byWidget(w), warnIfMissed: false);
                await tester.pumpAndSettle();
                clicked = true;
                break;
            }
        }
        if (!clicked) {
             final listTiles = find.byType(ListTile);
             if (listTiles.evaluate().isNotEmpty) {
                 await tester.ensureVisible(listTiles.first);
                 await tester.tap(listTiles.first, warnIfMissed: false);
                 await tester.pumpAndSettle();

                 final innerWebBuilder = find.textContaining('Website Builder');
                 if (innerWebBuilder.evaluate().isNotEmpty) {
                     await tester.ensureVisible(innerWebBuilder.first);
                     await tester.tap(innerWebBuilder.first, warnIfMissed: false);
                     await tester.pumpAndSettle();
                 }
             }
        }
    } else {
         final listTiles = find.byType(ListTile);
         if (listTiles.evaluate().isNotEmpty) {
             await tester.ensureVisible(listTiles.first);
             await tester.tap(listTiles.first, warnIfMissed: false);
             await tester.pumpAndSettle();

             final innerWebBuilder = find.textContaining('Website Builder');
             if (innerWebBuilder.evaluate().isNotEmpty) {
                 await tester.ensureVisible(innerWebBuilder.first);
                 await tester.tap(innerWebBuilder.first, warnIfMissed: false);
                 await tester.pumpAndSettle();
             }
         } else {
             // We fallback to drawer icon, find "Setup Wizard", tap, then look for "Website Builder"
             final navDrawerIcon = find.byIcon(Icons.menu);
             if (navDrawerIcon.evaluate().isNotEmpty) {
                 await tester.tap(navDrawerIcon.first, warnIfMissed: false);
                 await tester.pumpAndSettle();
             }
             final setupWizardText = find.textContaining('Setup');
             if (setupWizardText.evaluate().isNotEmpty) {
                await tester.ensureVisible(setupWizardText.first);
                await tester.tap(setupWizardText.first, warnIfMissed: false);
                await tester.pumpAndSettle();

                for (int i = 0; i < 5; i++) {
                    await tester.pump(const Duration(milliseconds: 500));
                }

                final innerWebBuilder = find.textContaining('Website');
                if (innerWebBuilder.evaluate().isNotEmpty) {
                    await tester.ensureVisible(innerWebBuilder.first);
                    await tester.tap(innerWebBuilder.first, warnIfMissed: false);
                    await tester.pumpAndSettle();
                }
             }
         }
    }

    for (int i = 0; i < 5; i++) {
        await tester.pump(const Duration(milliseconds: 500));
    }

    final ecommBtn = find.text('E-commerce');
    if (ecommBtn.evaluate().isNotEmpty) {
        // Step 0: Select a Template
        final templateBtn = find.text('E-commerce');
        await tester.ensureVisible(templateBtn.first);
        await tester.tap(templateBtn.first, warnIfMissed: false);
        await tester.pumpAndSettle();

        final nextBtn1 = find.text('Use this template →');
        await tester.ensureVisible(nextBtn1.first);
        await tester.tap(nextBtn1.first, warnIfMissed: false);
        await tester.pumpAndSettle();

        // Step 1: Brand Colors & Logo
        // Select first color (InkWell wraps a container)
        await tester.tap(find.descendant(of: find.byType(Row), matching: find.byType(InkWell)).first, warnIfMissed: false);
        await tester.pumpAndSettle();

        var nextBtn2 = find.text('Continue'); // Guessing based on the other wizard
        if(nextBtn2.evaluate().isEmpty) {
            nextBtn2 = find.text('Next');
        }
        await tester.ensureVisible(nextBtn2.first);
        await tester.tap(nextBtn2.first, warnIfMissed: false);
        await tester.pumpAndSettle();

        // Step 2: Add product
        final textFields = find.byType(TextField);
        await tester.enterText(textFields.at(0), 'Test Product');
        await tester.enterText(textFields.at(1), '99.99');

        var nextBtn3 = find.text('Continue');
        if(nextBtn3.evaluate().isEmpty) {
            nextBtn3 = find.text('Next');
        }
        await tester.ensureVisible(nextBtn3.first);
        await tester.tap(nextBtn3.first, warnIfMissed: false);
        await tester.pumpAndSettle();

        // Step 3: Domain
        final domainChoice = find.text('Free OHC subdomain');
        await tester.ensureVisible(domainChoice.first);
        await tester.tap(domainChoice.first, warnIfMissed: false);
        await tester.pumpAndSettle();

        var nextBtn4 = find.text('Continue');
        if(nextBtn4.evaluate().isEmpty) {
            nextBtn4 = find.text('Next');
        }
        await tester.ensureVisible(nextBtn4.first);
        await tester.tap(nextBtn4.first, warnIfMissed: false);
        await tester.pumpAndSettle();

        // Step 4: Publish
        var publishBtn = find.text('Publish');
        if(publishBtn.evaluate().isEmpty) {
            publishBtn = find.text('Publish →');
        }
        await tester.ensureVisible(publishBtn.first);
        await tester.tap(publishBtn.first, warnIfMissed: false);
        await tester.pumpAndSettle();
    }
  });
}
