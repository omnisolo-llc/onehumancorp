import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/material.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';
import 'package:app/providers/wizard_provider.dart';

void main() {
  group('BusinessSetupWizardScreen Environment Tests', () {
    testWidgets('Cloud mode shows External Integrations with Redis fields', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.cloud),
          ),
        ),
      );
      await tester.pump();

      // Skip to step 3 (External Integrations)
      final state = tester.state<ConsumerState<BusinessSetupWizardScreen>>(find.byType(BusinessSetupWizardScreen));
      final notifier = ProviderScope.containerOf(state.context).read(wizardProvider.notifier);

      notifier.nextStep();
      notifier.nextStep();
      notifier.nextStep();
      await tester.pump();

      expect(find.text('External Integrations'), findsOneWidget);
      expect(find.byKey(const Key('redisUrlField')), findsOneWidget);
      expect(find.byKey(const Key('dbUrlField')), findsOneWidget);
      expect(find.text('Local Environment Optimization'), findsNothing);
    });

    testWidgets('Standalone Desktop mode bypasses Redis and shows Local Environment Optimization', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(environmentMode: EnvironmentMode.standaloneDesktop),
          ),
        ),
      );
      await tester.pump();

      final state = tester.state<ConsumerState<BusinessSetupWizardScreen>>(find.byType(BusinessSetupWizardScreen));
      final notifier = ProviderScope.containerOf(state.context).read(wizardProvider.notifier);

      notifier.nextStep();
      notifier.nextStep();
      notifier.nextStep();
      await tester.pump();

      expect(find.text('Local Environment Optimization'), findsOneWidget);
      expect(find.text('External Integrations'), findsNothing);
      expect(find.byKey(const Key('redisUrlField')), findsNothing);
      expect(find.byKey(const Key('dbUrlField')), findsNothing);
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
      await tester.pump();

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
      await tester.pump();

      expect(find.text('Goals: '), findsOneWidget);
      expect(find.text('Deployment: null'), findsOneWidget);

      await tester.tap(find.byKey(const Key('toggleGoalBtn')));
      await tester.pump();
      expect(find.text('Goals: Build software'), findsOneWidget);

      await tester.tap(find.byKey(const Key('toggleGoalBtn')));
      await tester.pump();
      expect(find.text('Goals: '), findsOneWidget);

      await tester.tap(find.byKey(const Key('setDeploymentBtn')));
      await tester.pump();
      expect(find.text('Deployment: Cloud'), findsOneWidget);
    });
  });
}
