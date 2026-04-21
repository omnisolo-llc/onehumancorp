import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/agent_hire_wizard_screen.dart';

void main() {
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

    // If API fails to load it still loads but empty list of providers

    // Test the button exists. The "Next" button might be disabled initially on step 0
    // so we just expect the wizard to load.
    expect(find.text('Next'), findsWidgets);
  });
}
