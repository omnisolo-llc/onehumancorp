import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/auth_service.dart';

void main() {
  Widget _wrap(Widget child, {List<Override> overrides = const []}) {
    return ProviderScope(
      overrides: overrides,
      child: MaterialApp(home: Scaffold(body: child)),
    );
  }

  testWidgets('BusinessSetupWizardScreen renders and navigates steps in Cloud Mode', (tester) async {
    await tester.pumpWidget(_wrap(const BusinessSetupWizardScreen()));
    await tester.pumpAndSettle();

    // Step 0: Welcome
    expect(find.textContaining('Welcome!'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Business Name, Industry
    expect(find.text('Business Name'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'My Bakery');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: What they sell
    expect(find.text('What do you sell?'), findsOneWidget);
    await tester.enterText(find.byType(TextField).last, 'Cake');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Payment Preferences
    expect(find.text('Payment Preferences (e.g. Stripe)'), findsOneWidget);
    await tester.enterText(find.byType(TextField).last, 'Stripe');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Admin/Deployment info
    expect(find.text('Admin Name'), findsOneWidget);
    expect(find.byType(TextField), findsNWidgets(3));
  });

  testWidgets('BusinessSetupNotifier covers all state mutations', (tester) async {
    final container = ProviderContainer();
    final notifier = container.read(businessSetupProvider.notifier);

    notifier.updateCompany('Test Co');
    expect(container.read(businessSetupProvider).companyName, 'Test Co');

    notifier.updateIndustry('Tech');
    expect(container.read(businessSetupProvider).industry, 'Tech');

    notifier.updateWhatTheySell('Software');
    expect(container.read(businessSetupProvider).whatTheySell, 'Software');

    notifier.updatePaymentPreferences('Paypal');
    expect(container.read(businessSetupProvider).paymentPreferences, 'Paypal');

    notifier.updateDeployment('Desktop');
    expect(container.read(businessSetupProvider).deployment, 'Desktop');
  });
}
