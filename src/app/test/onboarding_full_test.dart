import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/start') {
        return http.Response('{"status": "ok"}', 200);
      } else if (request.url.path == '/api/onboarding/launch') {
        return http.Response('{"status": "ok"}', 200);
      } else if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{"bio": "saved bio test"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Tap 'Start a Business' on the welcome screen
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Verify we are on the input screen
    expect(find.text('Welcome to OHC Smart Builder'), findsOneWidget);

    // We no longer trigger 'Required' because the mock returns 'saved bio test'
    // so the field is not empty! Let's clear the field first.
    await tester.enterText(find.byKey(Key('bio-input')), '');
    await tester.pumpAndSettle();

    // Ensure we scroll to the button
    await tester.ensureVisible(find.text('Build My Storefront'));

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the bio field
    expect(find.text('Required'), findsOneWidget);

    // Fill out the bio form
    await tester.enterText(find.byKey(Key('bio-input')), "I bake custom vegan cakes in Seattle. Maya's Cakes.");
    await tester.pumpAndSettle();

    // Select Template
    await tester.tap(find.byKey(Key('template-Classic')));
    await tester.pumpAndSettle();

    // Select Domain
    await tester.tap(find.byKey(Key('domain-custom')));
    await tester.pumpAndSettle();

    // Enter Initial Product details
    await tester.enterText(find.byKey(Key('product-name-input')), "Vegan Chocolate Cake");
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(Key('product-price-input')), "45.00");
    await tester.pumpAndSettle();

    // Ensure we scroll to the button
    await tester.ensureVisible(find.text('Build My Storefront'));

    // Tap to build my storefront
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect to be on Dashboard state
    expect(find.text('Storefront Generated!'), findsOneWidget);

    // Go to draft preview
    await tester.tap(find.text('Preview Site'));
    await tester.pumpAndSettle();

    // Expect to be on Draft state
    expect(find.text('Preview Mode'), findsOneWidget);
    expect(find.text('1-Tap Launch'), findsOneWidget);

    // Verify touch target for edit
    expect(find.byIcon(Icons.edit), findsOneWidget);

    // Launch store
    await tester.tap(find.text('1-Tap Launch'));
    await tester.pumpAndSettle();

    // Expect to be on Live state
    expect(find.text("You're Live!"), findsOneWidget);
    await tester.pumpAndSettle(const Duration(seconds: 1));
  });
}
