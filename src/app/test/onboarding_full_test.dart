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
    expect(find.text('Your Business'), findsOneWidget);

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the field
    expect(find.text('Required'), findsNWidgets(1));

    // Fill out the form
    await tester.enterText(find.byType(TextFormField), "I bake custom cakes in Chicago");
    await tester.pumpAndSettle();

    // Test that the form can be submitted when both fields are filled out properly.
    // Note: Since we need to interact with a DropdownButtonFormField and simulate an HTTP request
    // this test primarily asserts the UI validation and state transitions up to form submission.
  });
}
