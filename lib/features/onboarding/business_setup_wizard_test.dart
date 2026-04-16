import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'business_setup_wizard.dart';

void main() {
  testWidgets('BusinessSetupWizard renders welcome text', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizard())));
    expect(find.text('Your AI team, ready in minutes'), findsOneWidget);
  });

  testWidgets('BusinessSetupWizard steps are accessible', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizard())));

    // Check initial step
    expect(find.text('Your AI team, ready in minutes'), findsOneWidget);

    // Step 1 -> Step 2
    final continueButton = find.text('Continue').first;
    await tester.ensureVisible(continueButton);
    await tester.tap(continueButton, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 2 profile fields
    expect(find.text('Company name'), findsOneWidget);

    // Step 2 -> Step 3
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 3 goal fields
    expect(find.text('Automate customer support'), findsOneWidget);

    // Step 3 -> Step 4
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 4 deployment mode
    expect(find.text('Self-hosted Desktop'), findsWidgets);

    // Step 4 -> Step 5
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 5 admin
    expect(find.text('Email'), findsOneWidget);
    expect(find.text('Password'), findsOneWidget);

    // Step 5 -> Step 6
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 6 review
    expect(find.text('Launch My AI Team →'), findsOneWidget);
  });
}
