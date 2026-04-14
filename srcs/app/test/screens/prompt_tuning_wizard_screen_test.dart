import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/prompt_tuning_wizard_screen.dart';

void main() {
  testWidgets('PromptTuningWizardScreen navigates steps and updates state', (WidgetTester tester) async {
    final router = GoRouter(
      initialLocation: '/agents/123/tune',
      routes: [
        GoRoute(
          path: '/agents/:id/tune',
          builder: (context, state) => PromptTuningWizardScreen(agentId: state.pathParameters['id'] ?? ''),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard Placeholder')),
        )
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    expect(find.text('Tune Agent: 123'), findsOneWidget);
    expect(find.text('Personality & tone'), findsOneWidget);

    // Tap next to step 2
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Domain focus'), findsOneWidget);

    // Tap next to step 3
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Example interactions'), findsOneWidget);

    // Tap next to step 4
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Review & Save'), findsOneWidget);

    // Tap save
    await tester.ensureVisible(find.text('Save'));
    await tester.tap(find.text('Save'));
    await tester.pumpAndSettle();
  });
}
