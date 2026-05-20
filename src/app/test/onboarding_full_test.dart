import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Validation test', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pump();

    // Expect the validator message 'Required'
    expect(find.text('Required'), findsOneWidget);
  });
}
