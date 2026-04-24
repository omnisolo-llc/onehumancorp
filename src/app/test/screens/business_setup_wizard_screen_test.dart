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

    // Initial state
    expect(find.text('Your business, live in minutes'), findsOneWidget);
    expect(find.text('Get Started'), findsOneWidget);

    // Step 1: Business Type
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();

    expect(find.text('What kind of business are you building?'), findsOneWidget);

    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();

    // Step 2: Business Profile
    expect(find.text('Tell us about your business'), findsOneWidget);
    expect(find.byType(TextFormField), findsNWidgets(2)); // Company Name, Description

    await tester.enterText(find.byType(TextFormField).first, 'Test Company');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 3: What do you sell
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
    expect(find.byType(TextFormField), findsNWidgets(3)); // Admin Name, Admin Email, Admin Password

    await tester.enterText(find.byType(TextFormField).at(0), 'Admin');
    await tester.enterText(find.byType(TextFormField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextFormField).at(2), 'password');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 6: Review
    expect(find.text('Review & Launch'), findsOneWidget);
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
    expect(container.read(businessSetupProvider).step, 4);

    notifier.updateCompany('NewCo');
    expect(container.read(businessSetupProvider).companyName, 'NewCo');

    notifier.updateDescription('Tech');
    expect(container.read(businessSetupProvider).businessDescription, 'Tech');

    notifier.updateBusinessType('L');
    expect(container.read(businessSetupProvider).businessType, 'L');

    notifier.toggleWhatYouSell('Support');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Support'), true);

    notifier.toggleWhatYouSell('Support');
    expect(container.read(businessSetupProvider).whatYouSell.contains('Support'), false);

    notifier.updatePaymentMethod('Desktop');
    expect(container.read(businessSetupProvider).paymentMethod, 'Desktop');

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
      await tester.tap(find.text(i == 0 ? 'Get Started' : 'Continue'));
      await tester.pumpAndSettle();
    }

    await tester.tap(find.text('Launch My Business →'));
    await tester.pumpAndSettle();

    // As auth is null, the API is bypassed and we should navigate to /dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });

}
