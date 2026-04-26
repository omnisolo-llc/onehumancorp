import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'business_setup_wizard.dart';

void main() {
  testWidgets('BusinessSetupWizard renders welcome text', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizard())));
    expect(find.text('Your AI team, ready in minutes'), findsOneWidget);
  });

  testWidgets('BusinessSetupWizard steps are accessible and state updates', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizard())));

    // Check initial step
    expect(find.text('Your AI team, ready in minutes'), findsOneWidget);

    // Step 1 -> Step 2
    final continueButton = find.text('Continue').first;
    await tester.ensureVisible(continueButton);
    await tester.tap(continueButton, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 2 profile fields
    expect(find.text('Company name'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.pumpAndSettle();

    // Step 2 -> Step 3
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 3 goal fields
    expect(find.text('Automate customer support'), findsOneWidget);
    await tester.tap(find.text('Automate customer support'));
    await tester.pumpAndSettle();

    // Step 3 -> Step 4
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 4 deployment mode
    expect(find.text('Self-hosted Desktop'), findsWidgets);
    await tester.tap(find.text('Self-hosted Desktop').first);
    await tester.pumpAndSettle();

    // Step 4 -> Step 5
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 5 admin
    expect(find.text('Email'), findsOneWidget);
    expect(find.text('Password'), findsOneWidget);

    await tester.enterText(find.byType(TextField).at(0), 'Admin Name');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'password');
    await tester.pumpAndSettle();

    // Step 5 -> Step 6
    await tester.ensureVisible(find.text('Continue').first);
    await tester.tap(find.text('Continue').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 6 review
    expect(find.text('Launch My AI Team →'), findsOneWidget);
    expect(find.text('Test Company'), findsOneWidget);
    expect(find.text('Self-hosted Desktop'), findsOneWidget);
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

    notifier.updateCompanyName('Acme');
    expect(container.read(businessSetupProvider).companyName, 'Acme');

    notifier.updateIndustry('Healthcare');
    expect(container.read(businessSetupProvider).industry, 'Healthcare');

    notifier.updateSize('L');
    expect(container.read(businessSetupProvider).size, 'L');

    notifier.updateLanguage('Spanish');
    expect(container.read(businessSetupProvider).language, 'Spanish');

    notifier.toggleGoal('Custom');
    expect(container.read(businessSetupProvider).goals.contains('Custom'), isTrue);

    notifier.toggleGoal('Custom');
    expect(container.read(businessSetupProvider).goals.contains('Custom'), isFalse);

    notifier.updateDeploymentMode('Mobile-only');
    expect(container.read(businessSetupProvider).deploymentMode, 'Mobile-only');

    notifier.updateAdminName('Admin');
    expect(container.read(businessSetupProvider).adminName, 'Admin');

    notifier.updateAdminEmail('admin@acme.com');
    expect(container.read(businessSetupProvider).adminEmail, 'admin@acme.com');

    notifier.updateAdminPassword('secr3t');
    expect(container.read(businessSetupProvider).adminPassword, 'secr3t');
  });
}
