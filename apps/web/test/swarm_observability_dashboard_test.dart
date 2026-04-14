import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

// Adjust import path if needed based on the monorepo structure,
// using relative path for simplicity as defined in the mission
import '../lib/swarm_observability_dashboard.dart';

void main() {
  testWidgets('SwarmObservabilityDashboard renders correctly with glassmorphism and animations', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityDashboard(),
        ),
      ),
    );

    // Verify text elements
    expect(find.text('Swarm Intelligence Status'), findsOneWidget);
    expect(find.text('All Agents Healthy'), findsOneWidget);

    // Verify BackdropFilter for glassmorphism
    expect(
      find.descendant(
        of: find.byType(SwarmObservabilityDashboard),
        matching: find.byType(BackdropFilter),
      ),
      findsOneWidget,
    );

    // Verify ScaleTransition for micro-animation
    expect(
      find.descendant(
        of: find.byType(SwarmObservabilityDashboard),
        matching: find.byType(ScaleTransition),
      ),
      findsOneWidget,
    );

    // Instead of pumpAndSettle (which times out because the animation is repeating),
    // we can just pump once to advance the clock.
    await tester.pump(const Duration(milliseconds: 500));
  });
}
