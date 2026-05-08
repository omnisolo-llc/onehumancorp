import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
  group('BusinessSetupWizardScreen Environment Tests', () {
    Future<void> navigateToStep4(WidgetTester tester) async {
      // 1. Welcome Screen
      final emailField = find.byKey(const Key('signupEmailField'));
      await tester.ensureVisible(emailField);
      await tester.enterText(emailField, 'test@test.com');
      await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pw');
      final signupBtn = find.byKey(const Key('signupBtn'));
      await tester.ensureVisible(signupBtn);
      await tester.tap(signupBtn);
      await tester.pump(const Duration(milliseconds: 500));

      // 2. Business Profile Screen
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // 3. Goal Selection Screen
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));
    }

    testWidgets('Cloud mode shows External Integrations with Redis fields', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
          ),
        ),
      );

      await navigateToStep4(tester);

      // Verify Cloud specifics
      expect(find.text('External Integrations'), findsOneWidget);
      expect(find.byKey(const Key('redisUrlField')), findsOneWidget);
      expect(find.byKey(const Key('dbUrlField')), findsOneWidget);

      // Verify Standalone specifics are absent
      expect(find.text('Local Environment Optimization'), findsNothing);
      expect(find.text('Bypassing Cloud Dependencies'), findsNothing);
    });

    testWidgets('Standalone Desktop mode bypasses Redis and shows Local Environment Optimization', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.standaloneDesktop),
          ),
        ),
      );

      await navigateToStep4(tester);

      // Verify Standalone specifics
      expect(find.text('Local Environment Optimization'), findsOneWidget);
      expect(find.text('Bypassing Cloud Dependencies'), findsOneWidget);

      // Verify Cloud specifics are absent
      expect(find.text('External Integrations'), findsNothing);
      expect(find.byKey(const Key('redisUrlField')), findsNothing);
      expect(find.byKey(const Key('dbUrlField')), findsNothing);
    });
  });
}
