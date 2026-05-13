import 'package:app/providers/wizard_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('Onboarding E2E: Simplified AI Flow', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    // Wait for the app to load and find the signup fields
    final emailField = find.byKey(const Key('signupEmailField'));
    await tester.ensureVisible(emailField);
    await tester.enterText(emailField, 'test1@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pass1');

    // Tap Signup
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pumpAndSettle(const Duration(milliseconds: 500));

    // Welcome Screen -> Tap Start
    await tester.tap(find.byType(ElevatedButton));
    await tester.pumpAndSettle(const Duration(milliseconds: 500));

    // Basic Info Screen
    final companyNameField = find.byKey(const Key('companyNameField'));
    await tester.ensureVisible(companyNameField);
    await tester.enterText(companyNameField, 'Maya\'s Bakery');

    final categoryField = find.byKey(const Key('categoryField'));
    await tester.ensureVisible(categoryField);
    await tester.enterText(categoryField, 'Bakery');

    // Tap Next
    await tester.tap(find.text('Next →'));
    await tester.pumpAndSettle(const Duration(milliseconds: 500));

    // Image Upload Screen
    final imageUploadBtn = find.byKey(const Key('imageUploadBtn'));
    await tester.ensureVisible(imageUploadBtn);
    await tester.tap(imageUploadBtn);
    await tester.pumpAndSettle(const Duration(milliseconds: 500));

    // Tap Generate
    final generateBtn = find.byKey(const Key('launchAIBtn'));
    await tester.ensureVisible(generateBtn);
    await tester.tap(generateBtn);

    // Pump frames to simulate AI generation delay (3s) and dashboard transition (2s)
    await tester.pump(const Duration(seconds: 1));
    await tester.pump(const Duration(seconds: 1));
    await tester.pump(const Duration(seconds: 1));
    await tester.pump(const Duration(seconds: 1));
    await tester.pump(const Duration(seconds: 1));
    await tester.pumpAndSettle();

    // Verify transition to Dashboard
    expect(find.text('Dashboard'), findsWidgets);
  });
}
