import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';

void main() {
  testWidgets('GrowBusinessWizardScreen renders and navigates through steps', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const GrowBusinessWizardScreen(),
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

    // Initial state: Step 0
    expect(find.text('Grow my business'), findsOneWidget);
    expect(find.text('Let\'s grow your business'), findsOneWidget);
    expect(find.text('Welcome to the Growth Wizard. Based on your current stage, here are some suggestions.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Navigate to Step 1
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('1. Expand your catalog'), findsOneWidget);
    expect(find.text('Add Products Now'), findsOneWidget);

    // Navigate to Step 2
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('2. Connect your socials'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);

    // Navigate to Step 3
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('3. Engage your audience'), findsOneWidget);
    expect(find.text('Send Campaign'), findsOneWidget);
    expect(find.text('Done'), findsOneWidget);

    // Go back to Step 2
    await tester.tap(find.text('Back'));
    await tester.pumpAndSettle();
    expect(find.text('2. Connect your socials'), findsOneWidget);

    // Go to Step 3 and complete
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Done'));
    await tester.pump(); // Start animation
    await tester.pump(const Duration(seconds: 1)); // Wait for simulated delay
    await tester.pumpAndSettle();

    // Verify redirection to Dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });

  test('GrowBusinessNotifier state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(growBusinessProvider.notifier);

    expect(container.read(growBusinessProvider).step, 0);

    notifier.nextStep();
    expect(container.read(growBusinessProvider).step, 1);

    notifier.nextStep();
    expect(container.read(growBusinessProvider).step, 2);

    notifier.previousStep();
    expect(container.read(growBusinessProvider).step, 1);

    notifier.previousStep();
    expect(container.read(growBusinessProvider).step, 0);

    notifier.previousStep(); // Should not go below 0
    expect(container.read(growBusinessProvider).step, 0);

    notifier.nextStep();
    notifier.nextStep();
    notifier.nextStep();
    expect(container.read(growBusinessProvider).step, 3);

    notifier.nextStep(); // Should not go above 3
    expect(container.read(growBusinessProvider).step, 3);
  });
}
