import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/settings_service.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates new steps correctly', (WidgetTester tester) async {
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

    // Step 0: Welcome
    expect(find.text('Your business, live in minutes.'), findsOneWidget);
    expect(find.text('Get Started'), findsOneWidget);
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();

    // Step 1: Business Type
    expect(find.text('What type of business are you building?'), findsOneWidget);
    expect(find.text('Online Store'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Name & Description
    expect(find.text('What is your business called?'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Maya Cakes');
    await tester.enterText(find.byType(TextField).at(1), 'I bake custom cakes');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: What do you sell?
    expect(find.text('What do you sell?'), findsOneWidget);
    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payments
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    await tester.tap(find.text('Both'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Admin Account
    expect(find.text('Create your admin account'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Maya');
    await tester.enterText(find.byType(TextField).at(1), 'maya@cakes.com');
    await tester.enterText(find.byType(TextField).at(2), 'password');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 6: Review & Launch
    expect(find.text('You are ready to launch!'), findsOneWidget);
    expect(find.text('Maya Cakes'), findsOneWidget);
    expect(find.text('Online Store'), findsOneWidget);
    expect(find.text('Physical products'), findsOneWidget);
    expect(find.text('Both'), findsOneWidget);
    expect(find.text('maya@cakes.com'), findsOneWidget);
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

    notifier.nextStep(); // 1
    notifier.nextStep(); // 2
    notifier.nextStep(); // 3
    notifier.nextStep(); // 4
    notifier.nextStep(); // 5
    notifier.nextStep(); // 6

    expect(container.read(businessSetupProvider).step, 6);

    notifier.updateBusinessType('Online Store');
    expect(container.read(businessSetupProvider).businessType, 'Online Store');

    notifier.updateBusinessName('NewCo');
    expect(container.read(businessSetupProvider).businessName, 'NewCo');

    notifier.updateBusinessDescription('Description');
    expect(container.read(businessSetupProvider).businessDescription, 'Description');

    notifier.toggleWhatDoYouSell('Physical products');
    expect(container.read(businessSetupProvider).whatDoYouSell.contains('Physical products'), true);

    notifier.toggleWhatDoYouSell('Physical products');
    expect(container.read(businessSetupProvider).whatDoYouSell.contains('Physical products'), false);

    notifier.updatePayments('Both');
    expect(container.read(businessSetupProvider).payments, 'Both');

    notifier.updateAdminName('Admin');
    expect(container.read(businessSetupProvider).adminName, 'Admin');

    notifier.updateAdminEmail('admin@example.com');
    expect(container.read(businessSetupProvider).adminEmail, 'admin@example.com');

    notifier.updateAdminPassword('secr3t');
    expect(container.read(businessSetupProvider).adminPassword, 'secr3t');
  });

  testWidgets('BusinessSetupWizardScreen launch bypasses API and routes to dashboard if no user is set', (WidgetTester tester) async {
    // We add a minimal go_router configuration so that GoRouter.of(context) does not throw.
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard')),
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
      if (i == 0) {
        await tester.tap(find.text('Get Started'));
      } else {
        await tester.tap(find.text('Next'));
      }
      await tester.pumpAndSettle();
    }

    await tester.tap(find.text('Launch My Business →'));
    await tester.pumpAndSettle();

    // As auth is null, the API is bypassed and we should navigate to /dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });
}
