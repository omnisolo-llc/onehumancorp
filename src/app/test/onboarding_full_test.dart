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
    expect(find.text('Setup Assistant'), findsOneWidget);

    // Fill out the bio form
    await tester.enterText(find.byKey(Key('bio-input')), "I bake custom vegan cakes in Seattle. Maya's Cakes.");
    await tester.pumpAndSettle();

    // Test that the form can be submitted by tapping the send button.
    // The send button is an IconButton with Icons.send
    await tester.tap(find.byIcon(Icons.send));
    await tester.pumpAndSettle();
  });
}
