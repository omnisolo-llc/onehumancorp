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
        return http.Response('{"businessType": "saved bio test"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Already on input screen
    await tester.pumpAndSettle();

    // Verify we are on the input screen
    expect(find.text('What do you do?'), findsOneWidget);

    // We no longer trigger 'Required' because the mock returns 'saved bio test'
    // so the field is not empty! Let's clear the field first.
    await tester.enterText(find.byKey(Key('type-input')), '');
    await tester.pumpAndSettle();

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the bio field
    expect(find.text('Required'), findsOneWidget);

    // Fill out the type form
    await tester.enterText(find.byKey(Key('type-input')), "Sell custom cakes");
    await tester.pumpAndSettle();

    // Tap Next to go to Name step
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('What\'s the name of your business?'), findsOneWidget);

    // Fill out the name form
    await tester.enterText(find.byKey(Key('name-input')), "Maya's Cakes");
    await tester.pumpAndSettle();

    // Tap Next to go to Niche step
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('What\'s your niche?'), findsOneWidget);

    // Fill out the niche
    await tester.enterText(find.byKey(Key('niche-input')), "I bake custom vegan cakes");
    await tester.pumpAndSettle();

    // Tap to build my storefront
    await tester.tap(find.text('Generate Draft'));
    await tester.pumpAndSettle();

    // Expect to be on Draft state directly
    expect(find.text('Looks Great!'), findsOneWidget);
    expect(find.text('Publish Now'), findsOneWidget);

    // Verify touch target for edit
    expect(find.byIcon(Icons.edit), findsOneWidget);

    // Launch store
    await tester.tap(find.text('Publish Now'));
    await tester.pumpAndSettle();

    // Expect to be on Live state
    expect(find.text("You're Live!"), findsOneWidget);
    await tester.pumpAndSettle(const Duration(seconds: 1));
  });
}
