import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';

void main() {
  testWidgets('Onboarding E2E: Conversational Intake Flow', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    // 1. Welcome Screen
    final emailField = find.byKey(const Key('signupEmailField'));
    await tester.ensureVisible(emailField);
    await tester.enterText(emailField, 'test1@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pass1');
    final signupBtn = find.byKey(const Key('signupBtn'));
    await tester.ensureVisible(signupBtn);
    await tester.tap(signupBtn);
    await tester.pump(const Duration(milliseconds: 500));

    // 2. Intake Screen
    final intentField = find.byKey(const Key('intentField'));
    await tester.ensureVisible(intentField);
    await tester.enterText(intentField, 'I want to sell vegan cakes in Portland.');

    final generateBtn = find.byKey(const Key('generateBtn'));
    await tester.ensureVisible(generateBtn);
    await tester.tap(generateBtn);

    // Pump to show generating screen
    await tester.pump();
    expect(find.text('Designing storefront...'), findsOneWidget);

    // Wait for AI Generation simulation (3 seconds)
    await tester.pump(const Duration(seconds: 4));

    // 3. Review Screen
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Generated Business'), findsOneWidget);
    expect(find.text('Flagship Product'), findsOneWidget);

    final launchBtn = find.byKey(const Key('launchAIBtn'));
    await tester.ensureVisible(launchBtn);

    // Pulse animation runs, so we pump instead of pumpAndSettle
    await tester.tap(launchBtn);
    await tester.pump(const Duration(seconds: 2));

    // 4. Checklist Screen
    expect(find.text("You're set up!"), findsOneWidget);

    final goDashBtn = find.text('Go to Dashboard');
    await tester.ensureVisible(goDashBtn);
    await tester.tap(goDashBtn);

    // Pump and wait for Dashboard replacement
    await tester.pumpAndSettle();

    // Check for dashboard
    expect(find.text("Dashboard"), findsOneWidget);
  });
}
