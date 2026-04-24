import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial state
    expect(find.text('Your business, live in minutes'), findsOneWidget);
    expect(find.text('No coding required. AI agents will set up your entire backend.'), findsOneWidget);
    expect(find.text('Get Started'), findsOneWidget);

    // Step 1: Business Profile
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();

    expect(find.text('What kind of business are you building?'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle(); // BusinessType auto-navigates to next step

    // Step 2: Company Name
    expect(find.text('Tell us about your business'), findsOneWidget);
    expect(find.byType(TextField), findsNWidgets(2)); // Name, Description

    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.enterText(find.byType(TextField).last, 'A test company description');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 3: What are you selling
    expect(find.text('What do you sell?'), findsOneWidget);

    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 4: Payments
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 5: Administrator account
    expect(find.text('Administrator account'), findsOneWidget);
    expect(find.byType(TextField), findsNWidgets(3)); // Admin Name, Admin Email, Admin Password

    await tester.enterText(find.byType(TextField).at(0), 'Admin');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'password');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    expect(find.text('Launch My Business →'), findsOneWidget);
  });

  test('BusinessSetupNotifier covers all state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(businessSetupProvider.notifier);

    expect(container.read(businessSetupProvider).step, 0);

    notifier.nextStep();
    expect(container.read(businessSetupProvider).step, 1);

    notifier.prevStep();
    expect(container.read(businessSetupProvider).step, 0);

    notifier.prevStep();
    expect(container.read(businessSetupProvider).step, 0);

    notifier.nextStep();
    notifier.nextStep();
    notifier.nextStep();
    notifier.nextStep();

    notifier.nextStep();
    expect(container.read(businessSetupProvider).step, 5);

    notifier.updateCompany('NewCo');
    expect(container.read(businessSetupProvider).companyName, 'NewCo');

    notifier.updateDescription('NewDesc');
    expect(container.read(businessSetupProvider).businessDescription, 'NewDesc');

    notifier.toggleWhatYouSell('Handmade Goods');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Handmade Goods'), true);

    notifier.toggleWhatYouSell('Handmade Goods');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Handmade Goods'), false);

    notifier.updatePaymentMethod('Online only');
    expect(container.read(businessSetupProvider).paymentMethod, 'Online only');

    notifier.updateAdminName('Admin');
    expect(container.read(businessSetupProvider).adminName, 'Admin');

    notifier.updateAdminEmail('admin@example.com');
    expect(container.read(businessSetupProvider).adminEmail, 'admin@example.com');

    notifier.updateAdminPassword('secr3t');
    expect(container.read(businessSetupProvider).adminPassword, 'secr3t');
  });
}
