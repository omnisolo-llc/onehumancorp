import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/prompt_tuning_wizard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  testWidgets('PromptTuningWizardScreen navigates steps and updates state', (WidgetTester tester) async {
    final mockApi = MockApiService();
    when(() => mockApi.tuneAgent(any(), any(), any(), any())).thenAnswer((_) async {});

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
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    expect(find.text('Tune Agent: 123'), findsOneWidget);
    expect(find.text('Personality & tone'), findsOneWidget);

    // Test tone selection
    await tester.ensureVisible(find.text('Detailed'));
    await tester.tap(find.text('Detailed'));
    await tester.pump();

    // Tap next to step 2
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Domain focus'), findsOneWidget);

    // Test domain focus toggle
    await tester.ensureVisible(find.text('Only discuss business'));
    await tester.tap(find.text('Only discuss business'));
    await tester.pump();

    // Tap next to step 3
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Example interactions'), findsOneWidget);

    // Test adding an example
    await tester.ensureVisible(find.text('Add Example'));
    await tester.tap(find.text('Add Example'));
    await tester.pump();
    expect(find.text('Sample Q'), findsOneWidget);

    // Tap next to step 4
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Review & Save'), findsOneWidget);

    // Tap save. It should call tuneAgent
    await tester.ensureVisible(find.text('Save'));
    await tester.tap(find.text('Save'));
    await tester.pumpAndSettle();

    verify(() => mockApi.tuneAgent('123', any(), 'Detailed', ['Only discuss business'])).called(1);
    expect(find.text('Your agent has been updated ✓'), findsOneWidget);
  });
}
