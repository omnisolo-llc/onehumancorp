import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';

void main() {
  testWidgets('Business Setup Wizard Minimal Flow E2E test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    // 1. Category Screen
    expect(find.text('What do you do?'), findsOneWidget);
    await tester.tap(find.text('Bake'));
    await tester.pump(const Duration(milliseconds: 500)); // Auto-advances

    // 2. Name Screen
    expect(find.text('What\'s the name of your business?'), findsOneWidget);
    await tester.enterText(find.byKey(const Key('companyNameField')), 'Maya\'s Bakes');
    await tester.tap(find.text('Next'));

    // In Flutter tests, async state changes often need multiple pumps to propagate.
    await tester.pump();
    await tester.pump();
    await tester.pump();

    // 3. Loading Screen
    expect(find.text('Generating your store...'), findsOneWidget);

    // Wait for the simulated API call (2 seconds)
    await tester.pump(const Duration(seconds: 2));
    await tester.pump(const Duration(milliseconds: 500));

    // 4. Dashboard Screen
    expect(find.text("Dashboard"), findsOneWidget);
    expect(find.text("Welcome Checklist"), findsOneWidget);
    expect(find.text("✅ Business live"), findsOneWidget);
    expect(find.text("⬜ Add 3 more products"), findsOneWidget);
  });
}
