import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/settings_service.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps in Cloud Mode', (WidgetTester tester) async {
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

    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Step 1: Business Profile
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.byType(TextField), findsNWidgets(2)); // Business Type, Company Name

    await tester.enterText(find.byType(TextField).first, 'Baker');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: What they sell
    expect(find.byType(TextField), findsOneWidget); // Products Services
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Template
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: First Product
    expect(find.byType(TextField), findsNWidgets(3));

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Domain
    expect(find.byType(TextField), findsNWidgets(1));

    await tester.enterText(find.byType(TextField).first, 'mydomain');
    await tester.pumpAndSettle();

    expect(find.text('Launch My AI Team →'), findsOneWidget);
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

    notifier.nextStep(); // 1
    notifier.nextStep(); // 2
    notifier.nextStep(); // 3
    notifier.nextStep(); // 4
    notifier.nextStep(); // 5
    expect(container.read(businessSetupProvider).step, 5);

    notifier.updateCompany('NewCo');
    expect(container.read(businessSetupProvider).companyName, 'NewCo');

    notifier.updateBusinessType('Baker');
    expect(container.read(businessSetupProvider).businessType, 'Baker');

    notifier.updateProductsServices('Cakes');
    expect(container.read(businessSetupProvider).productsServices, 'Cakes');

    notifier.updatePaymentPref('paypal');
    expect(container.read(businessSetupProvider).paymentPref, 'paypal');

    notifier.updateTemplateId('classic');
    expect(container.read(businessSetupProvider).templateId, 'classic');

    notifier.updateFirstProductName('Cake');
    expect(container.read(businessSetupProvider).firstProductName, 'Cake');

    notifier.updateFirstProductDesc('Yummy');
    expect(container.read(businessSetupProvider).firstProductDesc, 'Yummy');

    notifier.updateFirstProductPrice('10');
    expect(container.read(businessSetupProvider).firstProductPrice, '10');

    notifier.updateDomainName('baker');
    expect(container.read(businessSetupProvider).domainName, 'baker');
  });

  testWidgets('BusinessSetupWizardScreen launch bypasses API and routes if no user is set', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
        GoRoute(
          path: '/welcome_checklist',
          builder: (context, state) => const Scaffold(body: Text('Checklist')),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    for(int i = 0; i < 5; i++) {
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();
    }

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();

    expect(find.text('Checklist'), findsOneWidget);
  });
}