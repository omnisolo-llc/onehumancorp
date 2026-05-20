import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    // Tap 'Get Started' on the welcome screen
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();

    // Verify we are on the input screen
    expect(find.text('OHC Agent'), findsOneWidget);
    expect(find.text("Hi! Let's get your business set up. First, what's the name of your business?"), findsOneWidget);

    // Chat 1: Name
    await tester.enterText(find.byType(TextField), "Maya's Custom Cakes");
    await tester.tap(find.byIcon(Icons.send));
    await tester.pumpAndSettle(Duration(seconds: 1)); // Wait for delayed chat response

    // Verify chat response
    expect(find.text("Great name! What is your primary offering or goal? (e.g., selling physical products, booking services)"), findsOneWidget);

    // Chat 2: Offering
    await tester.enterText(find.byType(TextField), "Selling custom cakes");
    await tester.tap(find.byIcon(Icons.send));
    await tester.pumpAndSettle(Duration(seconds: 1)); // Wait for delayed chat response

    // Verify chat response
    expect(find.text("Got it. Finally, what's a good contact email or phone number for your business?"), findsOneWidget);

    // Chat 3: Contact Info
    // Note: Since this triggers the http request in submit(), we will stop here to avoid hanging the test without mocking http
    await tester.enterText(find.byType(TextField), "maya@example.com");
    await tester.tap(find.byIcon(Icons.send));
    await tester.pumpAndSettle(Duration(seconds: 1)); // Wait for delayed chat response and drain pending timers
  });
}
