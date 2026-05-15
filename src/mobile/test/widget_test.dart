import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:app/main.dart';

void main() {
  testWidgets('Onboarding flow test', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());

    expect(find.text('Setup'), findsOneWidget);

    // Initial delay for the first message
    await tester.pump(const Duration(milliseconds: 1000));
    await tester.pump(const Duration(milliseconds: 1500));
    await tester.pumpAndSettle();

    expect(find.text("First, what's the name of your business?"), findsOneWidget);

    // Enter business name
    await tester.enterText(find.byType(TextField), 'My Bakery');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pump(); // Start typing indicator
    await tester.pump(const Duration(milliseconds: 1000)); // Wait for agent
    await tester.pumpAndSettle();

    expect(find.text('My Bakery'), findsOneWidget);
    expect(find.text("Great name! What type of business is My Bakery?"), findsOneWidget);

    // Enter business type
    await tester.enterText(find.byType(TextField), 'Food');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 1000));
    await tester.pumpAndSettle();

    expect(find.text('Food'), findsOneWidget);
    expect(find.text("Got it. And what's your primary product or service?"), findsOneWidget);

    // Enter business product
    await tester.enterText(find.byType(TextField), 'Cakes');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 1000));
    await tester.pumpAndSettle();

    expect(find.text('Cakes'), findsOneWidget);
    expect(find.text("Perfect. I'm generating your custom storefront now..."), findsOneWidget);

    // Wait for generation
    await tester.pump(const Duration(seconds: 3));
    await tester.pumpAndSettle();

    // Verify storefront
    expect(find.text('My Bakery'), findsOneWidget);
    expect(find.text('Food'), findsOneWidget);
    expect(find.text('Cakes'), findsOneWidget);
    expect(find.text('Publish Store'), findsOneWidget);
  });
}
