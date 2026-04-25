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
    expect(find.text('Your business, live in minutes.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Step 1: Business Type
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Online Store'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Company Name & Description
    expect(find.byType(TextField), findsNWidgets(2));

    await tester.enterText(find.byType(TextField).first, 'My Bakery');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: What do you sell?
    expect(find.text('Physical products'), findsOneWidget);
    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payments
    expect(find.text('Online only'), findsOneWidget);
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Admin Account
    expect(find.byType(TextField), findsNWidgets(3));

    await tester.enterText(find.byType(TextField).first, 'Admin');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 6: Ready to Launch
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

    for(int i = 0; i < 6; i++) {
        notifier.nextStep();
    }
    expect(container.read(businessSetupProvider).step, 6);

    notifier.updateCompany('NewCo');
    expect(container.read(businessSetupProvider).companyName, 'NewCo');

    notifier.updateBusinessType('Baker');
    expect(container.read(businessSetupProvider).businessType, 'Baker');

    notifier.updateCompanyDescription('Best bakery');
    expect(container.read(businessSetupProvider).companyDescription, 'Best bakery');

    notifier.toggleSellItem('Physical products');
    expect(container.read(businessSetupProvider).sellItems.contains('Physical products'), true);

    notifier.toggleSellItem('Physical products');
    expect(container.read(businessSetupProvider).sellItems.contains('Physical products'), false);

    notifier.updatePaymentType('Online only');
    expect(container.read(businessSetupProvider).paymentType, 'Online only');

    notifier.updateAdminName('Admin');
    expect(container.read(businessSetupProvider).adminName, 'Admin');

    notifier.updateAdminEmail('admin@test.com');
    expect(container.read(businessSetupProvider).adminEmail, 'admin@test.com');

    notifier.updateAdminPassword('password');
    expect(container.read(businessSetupProvider).adminPassword, 'password');
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

    for(int i = 0; i < 6; i++) {
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();
    }

    await tester.tap(find.text('Launch My Business →'));
    await tester.pumpAndSettle();

    expect(find.text('Checklist'), findsOneWidget);
  });
}
