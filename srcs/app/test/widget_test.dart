import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('Business Setup Wizard E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );
    await tester.pump();

    // Scroll to avoid off-screen hit-test error
    final scrollView = find.byType(SingleChildScrollView);
    if (scrollView.evaluate().isNotEmpty) {
      await tester.drag(scrollView, const Offset(0, -300));
      await tester.pump();
    }

    // 0: Welcome
    expect(find.text('Welcome to One Human Corp'), findsOneWidget);
    await tester.enterText(find.byKey(const Key('signupEmailField')), 'test@example.com');
    await tester.enterText(find.byKey(const Key('signupPasswordField')), 'password123');

    await tester.ensureVisible(find.byKey(const Key('signupBtn')));
    await tester.tap(find.byKey(const Key('signupBtn')));
    await tester.pump();

    // 1: Profile
    expect(find.text('Business Profile'), findsOneWidget);
    await tester.enterText(find.byKey(const Key('companyNameField')), 'My Test Business');
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();

    // 2: Goal
    expect(find.text('What\'s your primary goal right now?'), findsOneWidget);
    await tester.ensureVisible(find.text('Sell products online'));
    await tester.tap(find.text('Sell products online'));
    await tester.pump();
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();

    // 3: AI Review
    expect(find.text('Review AI-Generated Draft'), findsOneWidget);
    await tester.ensureVisible(find.text('Modern'));
    await tester.tap(find.text('Modern'));
    await tester.pump();
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();

    // 4: Payment
    expect(find.text('Payment Setup'), findsOneWidget);
    await tester.ensureVisible(find.text('Defer for later'));
    await tester.tap(find.text('Defer for later'));
    await tester.pump();
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();

    // 5: Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    await tester.ensureVisible(find.text('Launch My AI Team'));
    await tester.tap(find.text('Launch My AI Team'));
    await tester.pump();

    for(int i = 0; i < 50; i++) {
        await tester.pump(const Duration(milliseconds: 100));
    }

    // Dashboard
    expect(find.text('Dashboard'), findsOneWidget);
    expect(find.text('Welcome Checklist'), findsOneWidget);
  });
}
