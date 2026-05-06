import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:app/main.dart';

void main() {
  testWidgets('Onboarding journey smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const OHCApp());

    // Verify that we start at the category selection screen
    expect(find.text('What do you do?'), findsOneWidget);
    expect(find.text('Bake'), findsOneWidget);

    // Tap the 'Bake' category
    await tester.tap(find.text('Bake'));
    await tester.pump();

    // Verify we moved to the name input screen
    expect(find.text("What's the name of your business?"), findsOneWidget);

    // Enter a business name
    await tester.enterText(find.byType(TextField), 'Maya Cakes');

    // Tap Continue
    await tester.tap(find.text('Continue'));
    await tester.pump();

    // Verify we moved to the loading screen
    expect(find.text('Generating your store...'), findsOneWidget);

    // Wait for the simulated AI loading (3 seconds)
    await tester.pump(const Duration(seconds: 3));

    // Verify we reached the dashboard
    expect(find.text("You're live!"), findsOneWidget);
    expect(find.text("Let's add your first item."), findsOneWidget);
  });
}
