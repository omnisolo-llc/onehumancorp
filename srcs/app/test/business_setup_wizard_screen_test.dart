import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/providers/wizard_provider.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
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

      // 2. Intake Screen
      final intentField = find.byKey(const Key('intentField'));
      await tester.ensureVisible(intentField);
      await tester.enterText(intentField, 'Testing intent');

      final generateBtn = find.byKey(const Key('generateBtn'));
      await tester.ensureVisible(generateBtn);
      await tester.tap(generateBtn);

      // Generating...
      await tester.pump();

      // Wait for AI
      await tester.pump(const Duration(seconds: 4));

      // 3. Review & Launch
      final launchBtn = find.byKey(const Key('launchAIBtn'));
      await tester.ensureVisible(launchBtn);
      await tester.tap(launchBtn);

      await tester.pump(const Duration(seconds: 2));

      // 4. Checklist
      expect(find.text('You\'re set up!'), findsOneWidget);
      expect(find.text('✅ Business live'), findsOneWidget);
      expect(find.text('⬜ Add 3 more products'), findsOneWidget);
      expect(find.text('⬜ Connect Instagram'), findsOneWidget);
      expect(find.text('⬜ Share your link with a friend'), findsOneWidget);

      // 5. Go to Dashboard
      await tester.tap(find.text('Go to Dashboard'));
      await tester.pumpAndSettle();

      expect(find.text('Dashboard'), findsOneWidget);
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
                      Text('Intent: ${state.intent}', key: const Key('intentText')),
                      ElevatedButton(
                        key: const Key('nextBtn'),
                        onPressed: () => notifier.nextStep(),
                        child: const Text('Next'),
                      ),
                      ElevatedButton(
                        key: const Key('setIntentBtn'),
                        onPressed: () => notifier.setIntent('New Intent'),
                        child: const Text('Set Intent'),
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
      expect(find.text('Intent: null'), findsOneWidget);

      // Trigger next step
      await tester.tap(find.byKey(const Key('nextBtn')));
      await tester.pump();
      expect(find.text('Step: 1'), findsOneWidget);

      // Update intent
      await tester.tap(find.byKey(const Key('setIntentBtn')));
      await tester.pump();
      expect(find.text('Intent: New Intent'), findsOneWidget);
    });
  });
}
