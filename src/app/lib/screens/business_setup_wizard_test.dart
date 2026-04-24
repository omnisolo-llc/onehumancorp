import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders welcome screen', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizardScreen())));
    expect(find.text('Your business, live in minutes'), findsOneWidget);
  });

  testWidgets('BusinessSetupWizardScreen steps navigation', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizardScreen())));

    // Step 0 -> 1
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();
    expect(find.text('What kind of business are you building?'), findsOneWidget);

    // Step 1 -> 2
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();
    expect(find.text('Tell us about your business'), findsOneWidget);

    // Step 2 -> 3
    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('What do you sell?'), findsOneWidget);

    // Step 3 -> 4
    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('How do you want to receive payments?'), findsOneWidget);

    // Step 4 -> 5
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('Administrator account'), findsOneWidget);

    // Step 5 -> 6
    await tester.enterText(find.byType(TextField).at(0), 'Admin Name');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'password123');
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Launch My Business →'), findsOneWidget);
  });

  test('BusinessSetupNotifier state transitions', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(businessSetupProvider.notifier);

    expect(container.read(businessSetupProvider).step, 0);

    notifier.nextStep();
    expect(container.read(businessSetupProvider).step, 1);

    notifier.prevStep();
    expect(container.read(businessSetupProvider).step, 0);

    notifier.updateBusinessType('Online Store');
    expect(container.read(businessSetupProvider).businessType, 'Online Store');

    notifier.updateCompany('Acme');
    expect(container.read(businessSetupProvider).companyName, 'Acme');

    notifier.updateDescription('A great company');
    expect(container.read(businessSetupProvider).businessDescription, 'A great company');

    notifier.toggleWhatYouSell('Physical products');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Physical products'), isTrue);

    notifier.updatePaymentMethod('Online only');
    expect(container.read(businessSetupProvider).paymentMethod, 'Online only');

    notifier.updateAdminName('Admin');
    expect(container.read(businessSetupProvider).adminName, 'Admin');

    notifier.updateAdminEmail('admin@acme.com');
    expect(container.read(businessSetupProvider).adminEmail, 'admin@acme.com');

    notifier.updateAdminPassword('secr3t');
    expect(container.read(businessSetupProvider).adminPassword, 'secr3t');
  });
}
