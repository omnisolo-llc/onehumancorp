import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/intake') {
        return http.Response('{"business_name": "Maya\'s Cakes", "business_type": "Retail", "categories": ["food"], "initial_products": [{"name": "Sample Product", "price": "10.00"}]}', 200);
      } else if (request.url.path == '/api/onboarding/start') {
        return http.Response('{"status": "ok"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Tap 'Start a Business' on the welcome screen
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Verify we are on the step1 screen
    expect(find.text("What's the name of your business?"), findsOneWidget);

    // Tap next without entering text to trigger validation
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the bio field
    expect(find.text('Required'), findsOneWidget);

    // Fill out the business name
    await tester.enterText(find.byKey(Key('bio-input')), "Maya's Cakes");
    await tester.pumpAndSettle();

    // Tap next
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify we are on step2 screen
    expect(find.text("What's your niche?"), findsOneWidget);

    // Fill out the niche
    await tester.enterText(find.byKey(Key('niche-input')), "I bake custom vegan cakes");
    await tester.pumpAndSettle();

    // Tap to build my storefront
    await tester.tap(find.text('Generate Draft'));
    await tester.pumpAndSettle();

    // Expect to be on Dashboard state
    expect(find.text('Looks Great!'), findsOneWidget);

    // Launch store
    await tester.tap(find.text('Publish Now'));
    await tester.pumpAndSettle();

    // Expect to be on Live state
    expect(find.text("You're Live!"), findsOneWidget);
  });
}
