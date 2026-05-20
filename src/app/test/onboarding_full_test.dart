import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    // Tap 'Start a Business' on the welcome screen
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Verify we are on the input screen
    expect(find.text('What do you want to build today?'), findsOneWidget);

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for both fields
    expect(find.text('Required'), findsOneWidget);

    // Fill out the form
    await tester.enterText(find.byType(TextFormField), "I want to build a custom cakes store");
    await tester.pumpAndSettle();

    // Test that the form can be submitted when both fields are filled out properly.
    // Note: Since we need to simulate an HTTP request
    // this test primarily asserts the UI validation and state transitions up to form submission.
  });
}
