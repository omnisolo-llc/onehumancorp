import 'package:app/providers/wizard_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';

void main() {
  testWidgets('Onboarding E2E: Standard Path (3-step)', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    // Welcome Screen
    final emailField = find.byKey(const Key('signupEmailField'));
    await tester.ensureVisible(emailField);
    await tester.enterText(emailField, 'test1@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pass1');

    final signupBtn = find.byKey(const Key('signupBtn'));
    await tester.ensureVisible(signupBtn);
    await tester.tap(signupBtn);
    await tester.pump(const Duration(milliseconds: 500));

    // Product Screen
    await tester.enterText(find.byKey(const Key('productNameField')), 'Prod 1');
    await tester.tap(find.byType(ElevatedButton).last); // Next
    await tester.pump(const Duration(milliseconds: 500));

    // Review & Launch Screen
    await tester.tap(find.byKey(const Key('launchAIBtn')));
    await tester.pump(const Duration(seconds: 3));
  });

  testWidgets('Onboarding E2E: Minimum Inputs', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    await tester.enterText(find.byKey(const Key('signupEmailField')), 'min@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'p');
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byType(ElevatedButton).last); // Next (skip product)
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byKey(const Key('launchAIBtn')));
    await tester.pump(const Duration(seconds: 3));
  });

  testWidgets('Onboarding E2E: Back and Forth Navigation', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: OHCApp()));

    await tester.enterText(find.byKey(const Key('signupEmailField')), 'back@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'p');
    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.enterText(find.byKey(const Key('productNameField')), 'A');
    await tester.tap(find.text('Back').last);
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Welcome to One Human Corp'), findsOneWidget);
  });
}
