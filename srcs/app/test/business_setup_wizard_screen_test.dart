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

    testWidgets('Full flow correctly navigates to new Product, Domain, and Checklist steps', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
          ),
        ),
      );

      await navigateToStep4(tester);

      // We are at step 3: External Integrations
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 4: Deployment
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 5: Administrator
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 6: Template Selection
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 7: Product Screen
      expect(find.text('Add your first product or service'), findsOneWidget);
      await tester.enterText(find.byKey(const Key('productNameField')), 'My Cool Product');
      await tester.enterText(find.byKey(const Key('productPriceField')), '99.99');
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 8: Domain Screen
      expect(find.text('Choose a Domain'), findsOneWidget);
      await tester.enterText(find.byKey(const Key('domainField')), 'mycustomdomain.ohc.app');
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 9: Review and Launch Screen
      expect(find.text('Review & Launch'), findsOneWidget);
      expect(find.text('My Cool Product'), findsOneWidget);
      expect(find.text('mycustomdomain.ohc.app'), findsOneWidget);

      // Launch!
      await tester.tap(find.text('Launch My AI Team'));
      await tester.pump(const Duration(seconds: 2));

      // Step 10: Checklist
      expect(find.text('You\'re set up!'), findsOneWidget);
      expect(find.text('✅ Business live'), findsOneWidget);
      expect(find.text('⬜ Add 3 more products'), findsOneWidget);
      expect(find.text('⬜ Connect Instagram'), findsOneWidget);
    });
  });
}
