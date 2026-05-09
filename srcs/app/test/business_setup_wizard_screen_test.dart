import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';
import 'package:app/providers/wizard_provider.dart';

void main() {
  group('BusinessSetupWizardScreen Environment Tests', () {
    Future<void> navigateToStep4(WidgetTester tester) async {
      // 1. Welcome Screen
      final emailField = find.byKey(const Key('signupEmailField'));
      await tester.ensureVisible(emailField);
      await tester.enterText(emailField, 'test@test.com');
      await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pw');
      final signupBtn = find.byKey(const Key('signupBtn'));
      await tester.ensureVisible(signupBtn);
      await tester.tap(signupBtn);
      await tester.pump(const Duration(milliseconds: 500));

      // 2. Business Profile Screen
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // 3. Goal Selection Screen
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));
    }

    testWidgets('Cloud mode shows External Integrations with Redis fields', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
          ),
        ),
      );

      await navigateToStep4(tester);

      // Verify Cloud specifics
      expect(find.text('External Integrations'), findsOneWidget);
      expect(find.byKey(const Key('redisUrlField')), findsOneWidget);
      expect(find.byKey(const Key('dbUrlField')), findsOneWidget);
      // Verify Standalone specifics are absent
      expect(find.text('Local Environment Optimization'), findsNothing);
      expect(find.text('Bypassing Cloud Dependencies'), findsNothing);
    });

    testWidgets('Standalone Desktop mode bypasses Redis and shows Local Environment Optimization', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.standaloneDesktop),
          ),
        ),
      );

      await navigateToStep4(tester);

      // Verify Standalone specifics
      expect(find.text('Local Environment Optimization'), findsOneWidget);
      expect(find.text('Bypassing Cloud Dependencies'), findsOneWidget);

      // Verify Cloud specifics are absent
      expect(find.text('External Integrations'), findsNothing);
      expect(find.byKey(const Key('redisUrlField')), findsNothing);
      expect(find.byKey(const Key('dbUrlField')), findsNothing);
    });

    testWidgets('Full flow correctly navigates to new Product, Domain, and Checklist steps', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
          ),
        ),
      );

      await navigateToStep4(tester);

      // We are at step 3: External Integrations
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 4: Deployment
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 5: Administrator
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 6: Template Selection
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 7: Product Screen
      expect(find.text('Add your first product or service'), findsOneWidget);
      await tester.enterText(find.byKey(const Key('productNameField')), 'My Cool Product');
      await tester.enterText(find.byKey(const Key('productPriceField')), '99.99');
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 8: Domain Screen
      expect(find.text('Choose a Domain'), findsOneWidget);
      await tester.enterText(find.byKey(const Key('domainField')), 'mycustomdomain.ohc.app');
      await tester.tap(find.text('Next'));
      await tester.pump(const Duration(milliseconds: 500));

      // Step 9: Review and Launch Screen
      expect(find.text('Review & Launch'), findsOneWidget);
      expect(find.text('My Cool Product'), findsOneWidget);
      expect(find.text('mycustomdomain.ohc.app'), findsOneWidget);

      // Launch!
      await tester.tap(find.text('Launch My AI Team'));
      await tester.pump(const Duration(seconds: 2));

      // Step 10: Checklist
      expect(find.text('You\'re set up!'), findsOneWidget);
      expect(find.text('✅ Business live'), findsOneWidget);
      expect(find.text('⬜ Add 3 more products'), findsOneWidget);
      expect(find.text('⬜ Connect Instagram'), findsOneWidget);
    });
  });


  group('WizardState tests', () {
    testWidgets('WizardProvider state persistence and updates', (WidgetTester tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: Consumer(
            builder: (context, ref, child) {
              final state = ref.watch(wizardProvider);
              final notifier = ref.read(wizardProvider.notifier);

              return MaterialApp(
                home: Scaffold(
                  body: Column(
                    children: [
                      Text('Step: ${state.currentStep}', key: const Key('stepText')),
                      Text('Company: ${state.companyName}', key: const Key('companyText')),
                      Text('Template: ${state.templateSelection}', key: const Key('templateText')),
                      ElevatedButton(
                        key: const Key('nextBtn'),
                        onPressed: () => notifier.nextStep(),
                        child: const Text('Next'),
                      ),
                      ElevatedButton(
                        key: const Key('updateProfileBtn'),
                        onPressed: () => notifier.updateBusinessProfile(companyName: 'Acme Corp'),
                        child: const Text('Update Profile'),
                      ),
                      ElevatedButton(
                        key: const Key('setTemplateBtn'),
                        onPressed: () => notifier.setTemplateSelection('Modern'),
                        child: const Text('Set Template'),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      );

      // Initial state
      expect(find.text('Step: 0'), findsOneWidget);
      expect(find.text('Company: null'), findsOneWidget);

      // Trigger next step
      await tester.tap(find.byKey(const Key('nextBtn')));
      await tester.pump();
      expect(find.text('Step: 1'), findsOneWidget);

      // Update business profile
      await tester.tap(find.byKey(const Key('updateProfileBtn')));
      await tester.pump();
      expect(find.text('Company: Acme Corp'), findsOneWidget);

      // Update template selection
      await tester.tap(find.byKey(const Key('setTemplateBtn')));
      await tester.pump();
      expect(find.text('Template: Modern'), findsOneWidget);
    });
  });



  group('Wizard Features testing', () {
    testWidgets('Wizard toggles goals and deployment preferences', (WidgetTester tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: Consumer(
            builder: (context, ref, child) {
              final state = ref.watch(wizardProvider);
              final notifier = ref.read(wizardProvider.notifier);

              return MaterialApp(
                home: Scaffold(
                  body: Column(
                    children: [
                      Text('Goals: ${state.goals.join(', ')}', key: const Key('goalsText')),
                      Text('Deployment: ${state.deploymentPreference}', key: const Key('deploymentText')),
                      ElevatedButton(
                        key: const Key('toggleGoalBtn'),
                        onPressed: () => notifier.toggleGoal('Build software'),
                        child: const Text('Toggle Goal'),
                      ),
                      ElevatedButton(
                        key: const Key('setDeploymentBtn'),
                        onPressed: () => notifier.setDeploymentPreference('Cloud'),
                        child: const Text('Set Deployment'),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      );

      // Initial state
      expect(find.text('Goals: '), findsOneWidget);
      expect(find.text('Deployment: null'), findsOneWidget);

      // Toggle goal on
      await tester.tap(find.byKey(const Key('toggleGoalBtn')));
      await tester.pump();
      expect(find.text('Goals: Build software'), findsOneWidget);

      // Toggle goal off
      await tester.tap(find.byKey(const Key('toggleGoalBtn')));
      await tester.pump();
      expect(find.text('Goals: '), findsOneWidget);

      // Set deployment
      await tester.tap(find.byKey(const Key('setDeploymentBtn')));
      await tester.pump();
      expect(find.text('Deployment: Cloud'), findsOneWidget);
    });
  });


  group('BusinessSetupWizardScreen Flow Tests', () {
    testWidgets('Wizard progression and Welcome Checklist in Dashboard', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
          ),
        ),
      );

      // 1. Welcome Screen
      final emailField = find.byKey(const Key('signupEmailField'));
      await tester.ensureVisible(emailField);
      await tester.enterText(emailField, 'test@test.com');
      await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pw');
      final signupBtn = find.byKey(const Key('signupBtn'));
      await tester.ensureVisible(signupBtn);
      await tester.tap(signupBtn);
      await tester.pump(const Duration(milliseconds: 500));

      // Go through all steps to step 7 (Review & Launch)
      // 2. Business Profile

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 3. Goal Selection

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 4. External Integrations

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 5. Deployment Preference

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 6. Administrator Account

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 7. Template Selection

      await tester.tap(find.text('Modern'));

      await tester.pump(const Duration(milliseconds: 100));

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 8. Product Configuration

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 9. Domain Selection

      await tester.tap(find.text('Next'));

      await tester.pump(const Duration(milliseconds: 500));

      // 10. Review & Launch -> Launch My AI Team
      final launchBtn = find.text('Launch My AI Team');
      await tester.ensureVisible(launchBtn);
      await tester.tap(launchBtn);
      await tester.pump(const Duration(milliseconds: 500));

      // After launch, step goes to 10, rendering DashboardScreen
      await tester.pump(const Duration(seconds: 2));
      // In the full flow testing logic, the launch button sets step to 10.
      expect(find.text('You\'re set up!'), findsOneWidget);
      expect(find.text('Here\'s what to do next:'), findsOneWidget);
      expect(find.text('✅ Business live'), findsOneWidget);
      expect(find.text('⬜ Add 3 more products'), findsOneWidget);
      expect(find.text('⬜ Connect Instagram'), findsOneWidget);
      expect(find.text('⬜ Share your link with a friend'), findsOneWidget);
    });
  });
}
