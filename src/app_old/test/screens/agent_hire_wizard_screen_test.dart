import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';
import 'package:ohc_app/services/settings_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('AgentHireWizardScreen test', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const Scaffold(body: AgentHireWizardScreen()),
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
    await tester.pumpAndSettle();

    expect(find.byType(AgentHireWizardScreen), findsOneWidget);
    expect(find.text('Next'), findsWidgets);
  });








  testWidgets('AgentHireWizardScreen advanced settings toggle test', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const Scaffold(body: AgentHireWizardScreen()),
        ),
      ],
    );

    // Let's create a fake provider override if necessary
    // However, the test might just be missing a pump.
    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );
    await tester.pumpAndSettle();

    for (int i=0; i<3; i++) {
        await tester.tap(find.text('Next').first);
        await tester.pumpAndSettle();
    }

    final context = tester.element(find.byType(AgentHireWizardScreen));
    final container = ProviderScope.containerOf(context);
    final notifier = container.read(clientSettingsProvider.notifier);

    // Toggle expertMode
    notifier.updateExpertMode(true);

    // We need to make sure the stream of state updates completes
    for(int i=0; i<5; i++){
        await tester.pump(const Duration(milliseconds: 500));
    }

    // Scroll to see it? Maybe it's off screen?
    // Let's drag up the ListView
    await tester.drag(find.byType(ListView).first, const Offset(0, -1000));
    await tester.pumpAndSettle();

    // We expect the Advanced Configuration to appear
    expect(find.text('Advanced Configuration'), findsOneWidget);
  });







}
