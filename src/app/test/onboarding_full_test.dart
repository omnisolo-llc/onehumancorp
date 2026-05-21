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
    expect(find.text('Welcome to OHC Smart Builder'), findsOneWidget);

    // Tap without entering text to trigger validation
    await tester.ensureVisible(find.text('Build My Storefront'));
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the fields
    expect(find.text('Required'), findsNWidgets(3));

    // Fill out the form
    await tester.enterText(find.byKey(Key('business-name-input')), "Maya's Cakes");
    await tester.enterText(find.byKey(Key('category-input')), "Bakery");
    await tester.enterText(find.byKey(Key('bio-input')), "I bake custom vegan cakes in Seattle.");
    await tester.pumpAndSettle();

    await tester.ensureVisible(find.text('Build My Storefront'));
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // We just assert UI state here up to form submission.
  });
}
