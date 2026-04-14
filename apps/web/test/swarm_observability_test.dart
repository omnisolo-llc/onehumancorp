import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

// Import the widget from the relative path based on execution environment
import '../lib/swarm_observability.dart';

void main() {
  group('SwarmObservabilityWidget Tests', () {
    testWidgets('renders active agents count correctly',
        (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityWidget(
              activeAgents: 42,
              statusMessage: 'All systems operational',
            ),
          ),
        ),
      );

      expect(find.text('Swarm Intelligence Status'), findsOneWidget);
      expect(find.text('42 Active Agents'), findsOneWidget);
      expect(find.text('All systems operational'), findsOneWidget);
    });

    testWidgets('contains Glassmorphism structural elements',
        (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityWidget(
              activeAgents: 10,
              statusMessage: 'Syncing memory...',
            ),
          ),
        ),
      );

      // Verify BackdropFilter for the blur effect
      expect(find.byType(BackdropFilter), findsOneWidget);

      // Verify animation container for the pulsing dot
      // The test environment adds an extra ScaleTransition around Scaffold for page transitions
      // We explicitly look for the ScaleTransition containing our colored pulsing dot
      expect(find.descendant(
        of: find.byType(SwarmObservabilityWidget),
        matching: find.byType(ScaleTransition),
      ), findsOneWidget);
    });
  });
}
