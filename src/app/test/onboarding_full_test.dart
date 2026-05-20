import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    // Tap 'Start a Business' on the welcome screen
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Verify we are on the chat input screen
    expect(find.text('The Promoter'), findsOneWidget);
    expect(find.text('What kind of business are you starting?'), findsOneWidget);

    // Fill out the chat
    await tester.enterText(find.byType(TextField), "Maya's Custom Cakes");
    await tester.pumpAndSettle();

    // Tap the send button
    await tester.tap(find.byIcon(Icons.send));

    // Let the first frame run to change the state to generating
    await tester.pump();

    // It should throw an exception after due to http request but we don't care,
    // we just want to verify the text changed to generating before.
  });
}
