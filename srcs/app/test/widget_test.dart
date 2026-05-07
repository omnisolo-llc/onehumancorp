import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:app/main.dart';

void main() {
  testWidgets('Onboarding journey E2E test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const OHCApp());

    // Verify that we start at the intake screen
    expect(find.text('What do you want to build today?'), findsOneWidget);

    // Enter a business intent
    await tester.enterText(find.byType(TextField), 'I sell vegan cakes');

    // Tap Continue
    await tester.tap(find.text('Continue'));
    await tester.pump();

    // Verify we moved to the loading screen
    expect(find.text('Designing storefront...'), findsOneWidget);

    // Wait for the simulated AI loading (3 seconds)
    await tester.pump(const Duration(seconds: 3));

    // Verify we reached the review screen
    expect(find.text('Review Your Business'), findsOneWidget);
    expect(find.text('Storefront Preview'), findsOneWidget);
    expect(find.text('Launch Business'), findsOneWidget);

    // Tap Launch Business
    await tester.tap(find.text('Launch Business'));
    await tester.pump();

    // Verify we reached the dashboard
    expect(find.text("Dashboard"), findsOneWidget);
    expect(find.text("Pending Agent Approvals"), findsOneWidget);
  });
}
