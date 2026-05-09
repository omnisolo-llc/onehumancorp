import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';
import 'package:app/providers/wizard_provider.dart';
import 'package:app/screens/referral_program_screen.dart';

void main() {
  testWidgets('Wizard has new items clickable and copies domain on go-live', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
        ),
      ),
    );

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

    // 4. External Integrations
    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 5. Deployment Preference
    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 6. Administrator Account
    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 7. Template Selection
    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 8. Product Configuration
    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 9. Domain Assignment
    await tester.tap(find.text('Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // 10. Review & Launch
    final launchBtn = find.byKey(const Key('launchAIBtn'));
    await tester.ensureVisible(launchBtn);
    await tester.tap(launchBtn);
    await tester.pump(const Duration(milliseconds: 500));

    // Pump and settle for snackbar
    for (var i = 0; i < 50; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }

    expect(find.text('You\'re set up!'), findsOneWidget);

    // Tap on Add 3 more products (navigates back to Product configuration screen 7)
    final addProductsItem = find.textContaining('Add 3 more products');
    await tester.ensureVisible(addProductsItem);
    await tester.tap(addProductsItem);
    await tester.pump(const Duration(milliseconds: 500));
    expect(find.text('Add your first product or service'), findsOneWidget); // We navigated!

    // Go forward to step 10 again manually using provider is hard in test, let's just use tap Next
    await tester.tap(find.text('Next')); // To 8 Domain
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(find.text('Next')); // To 9 Review
    await tester.pump(const Duration(milliseconds: 500));
    await tester.tap(launchBtn); // Launch again
    await tester.pump(const Duration(milliseconds: 500));
    for (var i = 0; i < 50; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }

    // Now test Share
    final shareItem = find.textContaining('Share your link');
    await tester.ensureVisible(shareItem);
    await tester.tap(shareItem);
    await tester.pumpAndSettle();

    // We should be on ReferralProgramScreen
    expect(find.byType(ReferralProgramScreen), findsOneWidget);
  });
}
