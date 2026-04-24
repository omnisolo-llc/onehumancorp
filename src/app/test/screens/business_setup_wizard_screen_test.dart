import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/settings_service.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          clientSettingsProvider.overrideWith(
            (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
              ClientSettings(backendUrl: 'http://localhost', standaloneMode: false),
            ),
          ),
        ],
        child: const MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial state (Step 0)
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Step 1: Business Type
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Business type'), findsOneWidget);
    expect(find.byType(ChoiceChip), findsNWidgets(6));

    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Business name & description
    expect(find.byType(TextField), findsNWidgets(2)); // Name, Description

    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: What do you sell?
    expect(find.text('What do you sell?'), findsOneWidget);
    expect(find.byType(CheckboxListTile), findsNWidgets(5));

    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payments
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsNWidgets(4));

    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Template
    expect(find.text('Template Selection'), findsOneWidget);
    expect(find.byType(ChoiceChip), findsNWidgets(4));

    await tester.tap(find.text('Modern'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 6: Product
    expect(find.text('First Product / Service'), findsOneWidget);
    expect(find.text('Magic Fill'), findsOneWidget);

    await tester.enterText(find.byType(TextField).first, 'Coffee');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 7: Domain & Go-Live
    expect(find.text('Domain & Go-Live'), findsOneWidget);
    expect(find.text('Launch My Business →'), findsOneWidget);
  });

  test('BusinessSetupNotifier covers all state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(businessSetupProvider.notifier);

    expect(container.read(businessSetupProvider).step, 0);

    notifier.nextStep();
    expect(container.read(businessSetupProvider).step, 1);

    notifier.updateBusinessType('Online Store');
    expect(container.read(businessSetupProvider).businessType, 'Online Store');

    notifier.updateCompany('NewCo');
    expect(container.read(businessSetupProvider).companyName, 'NewCo');

    notifier.updateDescription('Selling widgets');
    expect(container.read(businessSetupProvider).description, 'Selling widgets');

    notifier.toggleWhatYouSell('Physical products');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Physical products'), true);

    notifier.updatePayments('Online only');
    expect(container.read(businessSetupProvider).payments, 'Online only');

    notifier.updateTemplate('Modern');
    expect(container.read(businessSetupProvider).template, 'Modern');

    notifier.updateFirstProductName('Widget');
    expect(container.read(businessSetupProvider).firstProductName, 'Widget');

    notifier.updateFirstProductPrice('10');
    expect(container.read(businessSetupProvider).firstProductPrice, '10');

    notifier.updateDomain('testco');
    expect(container.read(businessSetupProvider).domain, 'testco');
  });
}
