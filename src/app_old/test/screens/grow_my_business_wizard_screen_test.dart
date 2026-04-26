import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/grow_my_business_wizard_screen.dart';
import 'package:go_router/go_router.dart';

void main() {
  testWidgets('GrowMyBusinessWizardScreen navigates through all steps', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const GrowMyBusinessWizardScreen(),
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

    // Step 0
    expect(find.text('Growth Strategies'), findsOneWidget);
    expect(find.text('Select a next step to grow your business'), findsOneWidget);

    // Next is disabled
    final nextButtonFinder = find.widgetWithText(ElevatedButton, 'Next');
    expect(tester.widget<ElevatedButton>(nextButtonFinder).enabled, isFalse);

    // Select strategy
    await tester.tap(find.text('Connect Instagram'));
    await tester.pumpAndSettle();
    expect(tester.widget<ElevatedButton>(nextButtonFinder).enabled, isTrue);

    // Step 1
    await tester.ensureVisible(nextButtonFinder);
    await tester.tap(nextButtonFinder);
    await tester.pumpAndSettle();
    expect(find.text('Confirm Action'), findsOneWidget);
    expect(find.text('You are about to start: Connect Instagram'), findsOneWidget);

    // Check advanced mode toggle
    final switchFinder = find.byType(Switch);
    expect(switchFinder, findsWidgets);
    await tester.tap(switchFinder.first);
    await tester.pumpAndSettle();

    // Back test
    await tester.tap(find.widgetWithText(OutlinedButton, 'Back'));
    await tester.pumpAndSettle();
    expect(find.text('Select a next step to grow your business'), findsOneWidget);
    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();

    // Execute
    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Execute'));
    await tester.tap(find.widgetWithText(ElevatedButton, 'Execute'));
    await tester.pump(); // start animation
    await tester.pump(const Duration(seconds: 1)); // complete delay
    await tester.pumpAndSettle(); // route to dashboard

    expect(find.text('Dashboard'), findsOneWidget);
  });

  test('GrowMyBusinessNotifier state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(growMyBusinessProvider.notifier);
    expect(container.read(growMyBusinessProvider).step, 0);

    notifier.nextStep();
    expect(container.read(growMyBusinessProvider).step, 1);

    notifier.previousStep();
    expect(container.read(growMyBusinessProvider).step, 0);

    notifier.updateStrategy('Run your first email campaign');
    expect(container.read(growMyBusinessProvider).selectedStrategy, 'Run your first email campaign');
  });
}
