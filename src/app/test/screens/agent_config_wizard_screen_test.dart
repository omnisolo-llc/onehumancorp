import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/agent_config_wizard_screen.dart';
import 'package:go_router/go_router.dart';

void main() {
  testWidgets('AgentConfigWizardScreen navigates through all steps', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const AgentConfigWizardScreen(),
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
    expect(find.text('Configure AI Agent'), findsOneWidget);
    expect(find.text('Choose an agent to add to your team'), findsOneWidget);

    // Next is disabled
    final nextButtonFinder = find.widgetWithText(ElevatedButton, 'Next');
    expect(tester.widget<ElevatedButton>(nextButtonFinder).enabled, isFalse);

    // Select agent
    await tester.tap(find.text('Customer Support'));
    await tester.pumpAndSettle();
    expect(tester.widget<ElevatedButton>(nextButtonFinder).enabled, isTrue);

    // Step 1
    await tester.ensureVisible(nextButtonFinder);
    await tester.tap(nextButtonFinder);
    await tester.pumpAndSettle();
    expect(find.text('What should this agent do?'), findsOneWidget);

    // Check advanced mode toggle
    final switchFinder = find.byType(Switch);
    expect(switchFinder, findsWidgets);
    await tester.tap(switchFinder.first);
    await tester.pumpAndSettle();

    // Toggle caps
    await tester.tap(find.text('Reply to customer messages'));
    await tester.pumpAndSettle();

    // Step 2
    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();
    expect(find.text('How often should this agent work?'), findsOneWidget);

    // Back test
    await tester.tap(find.widgetWithText(OutlinedButton, 'Back'));
    await tester.pumpAndSettle();
    expect(find.text('What should this agent do?'), findsOneWidget);
    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();

    // Change frequency
    await tester.drag(find.byType(Slider), const Offset(50, 0));
    await tester.pumpAndSettle();

    // Step 3
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();
    expect(find.text('Review & Activate'), findsOneWidget);
    expect(find.text('Agent: Customer Support'), findsOneWidget);
    expect(find.text('• Reply to customer messages'), findsOneWidget);

    // Activate
    await tester.tap(find.widgetWithText(ElevatedButton, 'Activate'));
    await tester.pump(); // Start simulation
    await tester.pump(const Duration(seconds: 1)); // Finish
    await tester.pumpAndSettle(); // Go router nav

    expect(find.text('Dashboard'), findsOneWidget);
  });

  test('AgentConfigNotifier state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(agentConfigProvider.notifier);
    expect(container.read(agentConfigProvider).step, 0);

    notifier.nextStep();
    expect(container.read(agentConfigProvider).step, 1);

    notifier.previousStep();
    expect(container.read(agentConfigProvider).step, 0);

    notifier.updateAgent('Order Manager');
    expect(container.read(agentConfigProvider).selectedAgent, 'Order Manager');

    notifier.toggleCapability('reply');
    expect(container.read(agentConfigProvider).canReplyMessages, isTrue);
    notifier.toggleCapability('reply');
    expect(container.read(agentConfigProvider).canReplyMessages, isFalse);

    notifier.toggleCapability('social');
    expect(container.read(agentConfigProvider).canPostSocial, isTrue);

    notifier.toggleCapability('desc');
    expect(container.read(agentConfigProvider).canWriteProductDesc, isTrue);

    notifier.toggleCapability('order');
    expect(container.read(agentConfigProvider).canSendOrderUpdates, isTrue);

    notifier.updateFrequency(3.0);
    expect(container.read(agentConfigProvider).frequency, 3.0);
  });
}
