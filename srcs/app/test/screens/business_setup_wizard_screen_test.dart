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
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Step 1: Business Profile
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.byType(TextField), findsNWidgets(2)); // Company Name, Industry
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget); // Size

    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Goal selection
    expect(find.text('Select Goals'), findsOneWidget);
    expect(find.byType(CheckboxListTile), findsNWidgets(5));

    await tester.tap(find.text('Support'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Deployment Preference (Cloud Mode)
    expect(find.text('Deployment Preference'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsNWidgets(3));
    expect(find.text('Standalone Mode Detected. Multi-tenant cloud databases and Redis configurations bypassed for local execution.'), findsNothing);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Administrator account
    expect(find.byType(TextField), findsNWidgets(3)); // Admin Name, Admin Email, Admin Password

    await tester.enterText(find.byType(TextField).at(0), 'Admin');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'password');
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

    notifier.updateIndustry('Tech');
    expect(container.read(businessSetupProvider).industry, 'Tech');

    notifier.updateSize('L');
    expect(container.read(businessSetupProvider).size, 'L');

    notifier.toggleGoal('Support');
    expect(container.read(businessSetupProvider).goals.contains('Support'), true);

    notifier.toggleGoal('Support');
    expect(container.read(businessSetupProvider).goals.contains('Support'), false);

    notifier.updateDeployment('Desktop');
    expect(container.read(businessSetupProvider).deployment, 'Desktop');

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

    for(int i = 0; i < 4; i++) {
      await tester.tap(find.text('Next'));
      await tester.pumpAndSettle();
    }

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();

    // As auth is null, the API is bypassed and we should navigate to /dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });

  testWidgets('BusinessSetupWizardScreen renders and navigates steps in Standalone Mode', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          clientSettingsProvider.overrideWith(
            (ref) => ClientSettingsNotifier(ref)..state = const AsyncValue.data(
              ClientSettings(backendUrl: 'http://localhost', standaloneMode: true),
            ),
          ),
        ],
        child: const MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Navigate to Step 3
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Deployment Preference (Standalone Mode)
    expect(find.text('Deployment Preference'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsNothing); // Should be hidden
    expect(find.text('Standalone Mode Detected. Multi-tenant cloud databases and Redis configurations bypassed for local execution.'), findsOneWidget); // Bypass message
  });
}
