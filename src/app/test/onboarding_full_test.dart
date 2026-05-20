import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    // To avoid overflow errors on small virtual test screens
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 3.0;

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));
    await tester.pumpAndSettle();

    // Verify we are on the input screen
    expect(find.text('What are you building today?'), findsOneWidget);

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the field
    expect(find.text('Required'), findsNWidgets(1));

    // Fill out the form
    await tester.enterText(find.byType(TextFormField), "Maya's Custom Cakes");
    await tester.pumpAndSettle();

    // Note: Since we need to simulate an HTTP request
    // this test primarily asserts the UI validation and state transitions up to form submission.

    // Reset physical size
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
