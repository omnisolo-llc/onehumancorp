import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/settings_service.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps', (
    WidgetTester tester,
  ) async {
    tester.view.physicalSize = const Size(1080, 1920);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          clientSettingsProvider.overrideWith(
            (ref) => ClientSettingsNotifier(ref)
              ..state = const AsyncValue.data(
                ClientSettings(
                  backendUrl: 'http://localhost',
                  standaloneMode: false,
                ),
              ),
          ),
        ],
        child: const MaterialApp(home: BusinessSetupWizardScreen()),
      ),
    );

    // Initial state (Step 0)
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Your business, live in minutes.'), findsOneWidget);
    expect(find.text('Continue'), findsOneWidget);

    // Step 1: Business Type
    await tester.tap(find.text('Continue'));
    await tester.pump(const Duration(milliseconds: 300));

    expect(find.text('Business Type'), findsOneWidget);
    // We changed RadioListTile to GestureDetector/Containers with icons and text
    expect(find.text('Online Store'), findsOneWidget);

    await tester.tap(find.text('Online Store'));
    await tester.pump(const Duration(milliseconds: 300));

    await tester.tap(find.text('Continue'));
    await tester.pump(const Duration(milliseconds: 300));

    // Step 2: Business name & description
    expect(
      find.byType(TextFormField),
      findsNWidgets(2),
    ); // Company Name, Description

    await tester.enterText(find.byType(TextFormField).first, 'Test Company');
    await tester.pump(const Duration(milliseconds: 300));

    await tester.tap(find.text('Continue'));
    await tester.pump(const Duration(milliseconds: 300));

    // Step 3: What do you sell?
    expect(find.text('What do you sell?'), findsOneWidget);
    expect(find.text('Physical products'), findsOneWidget);

    await tester.tap(find.text('Physical products'));
    await tester.pump(const Duration(milliseconds: 300));

    await tester.tap(find.text('Continue'));
    await tester.pump(const Duration(milliseconds: 300));

    // Step 4: Payments
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    expect(find.text('Online only'), findsOneWidget);

    await tester.tap(find.text('Online only'));
    await tester.pump(const Duration(milliseconds: 300));

    await tester.tap(find.text('Continue'));
    await tester.pump(const Duration(milliseconds: 300));

    // Step 5: Administrator account
    expect(find.text('Admin Account'), findsOneWidget);
    expect(
      find.byType(TextFormField),
      findsNWidgets(3),
    ); // Admin Name, Admin Email, Admin Password

    await tester.enterText(find.byType(TextFormField).at(0), 'Admin');
    await tester.enterText(find.byType(TextFormField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextFormField).at(2), 'password');
    await tester.pump(const Duration(milliseconds: 300));

    await tester.tap(find.text('Continue'));
    await tester.pump(const Duration(milliseconds: 300));

    // Step 6: Review
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Business Type: Online Store'), findsOneWidget);
    expect(find.text('Name: Test Company'), findsOneWidget);

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

    notifier.nextStep();
    expect(container.read(businessSetupProvider).step, 6);

    notifier.updateBusinessType('Online Store');
    expect(container.read(businessSetupProvider).businessType, 'Online Store');

    notifier.updateCompany('NewCo');
    expect(container.read(businessSetupProvider).companyName, 'NewCo');
    expect(
      container.read(businessSetupProvider).description,
      'NewCo provides amazing online store services to the community.',
    );

    notifier.updateDescription('Custom description');
    expect(
      container.read(businessSetupProvider).description,
      'Custom description',
    );

    notifier.toggleWhatYouSell('Physical products');
    expect(
      container
          .read(businessSetupProvider)
          .whatYouSell
          .contains('Physical products'),
      true,
    );

    notifier.toggleWhatYouSell('Physical products');
    expect(
      container
          .read(businessSetupProvider)
          .whatYouSell
          .contains('Physical products'),
      false,
    );

    notifier.updatePaymentMethod('Both');
    expect(container.read(businessSetupProvider).paymentMethod, 'Both');

    notifier.updateAdminName('Admin');
    expect(container.read(businessSetupProvider).adminName, 'Admin');

    notifier.updateAdminEmail('admin@example.com');
    expect(
      container.read(businessSetupProvider).adminEmail,
      'admin@example.com',
    );

    notifier.updateAdminPassword('secr3t');
    expect(container.read(businessSetupProvider).adminPassword, 'secr3t');
  });

  testWidgets(
    'BusinessSetupWizardScreen launch bypasses API and routes to dashboard if no user is set',
    (WidgetTester tester) async {
      tester.view.physicalSize = const Size(1080, 1920);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      // We add a minimal go_router configuration so that GoRouter.of(context) does not throw.
      final router = GoRouter(
        routes: [
          GoRoute(
            path: '/',
            builder: (context, state) => const BusinessSetupWizardScreen(),
          ),
          GoRoute(
            path: '/dashboard',
            builder:
                (context, state) => const Scaffold(body: Text('Dashboard')),
          ),
        ],
      );

      await tester.pumpWidget(
        ProviderScope(child: MaterialApp.router(routerConfig: router)),
      );

      for (int i = 0; i < 6; i++) {
        await tester.tap(find.text('Continue'));
        await tester.pump(const Duration(milliseconds: 300));
      }

      await tester.tap(find.text('Launch My Business →'));
      await tester.pump(const Duration(milliseconds: 300));
      await tester.pump(const Duration(seconds: 2));

      // As auth is null, the API is bypassed and we should navigate to /dashboard
      expect(find.text('Dashboard'), findsOneWidget);
    },
  );
}
