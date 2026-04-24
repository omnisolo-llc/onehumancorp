import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/sub_agent_queue_widget.dart';
import 'package:ohc_app/models/dashboard.dart';

void main() {
  testWidgets('SubAgentQueueWidget renders title and counts', (WidgetTester tester) async {
    final statuses = [
      const StatusBucket(status: 'pending', count: 5),
      const StatusBucket(status: 'in_progress', count: 3),
      const StatusBucket(status: 'completed', count: 10),
      const StatusBucket(status: 'failed', count: 2),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SubAgentQueueWidget(statuses: statuses),
        ),
      ),
    );

    await tester.pump(); // Start animation

    expect(find.text('Sub-Agent Orchestration Queue'), findsOneWidget);

    expect(find.text('Enqueued'), findsOneWidget);
    expect(find.text('5'), findsOneWidget);

    expect(find.text('Processing'), findsOneWidget);
    expect(find.text('3'), findsOneWidget);

    expect(find.text('Completed'), findsOneWidget);
    expect(find.text('10'), findsOneWidget);

    expect(find.text('Failed'), findsOneWidget);
    expect(find.text('2'), findsOneWidget);
  });

  testWidgets('SubAgentQueueWidget pulses Processing node', (WidgetTester tester) async {
    final statuses = [
      const StatusBucket(status: 'in_progress', count: 3),
    ];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SubAgentQueueWidget(statuses: statuses),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final scaleFinder = find.descendant(
      of: find.byWidgetPredicate((w) {
        if (w is Column) {
          return w.children.any((c) => c is Text && c.data == 'Processing');
        }
        return false;
      }),
      matching: find.byType(ScaleTransition),
    );

    final scaleTransitions = tester.widgetList<ScaleTransition>(scaleFinder);
    expect(scaleTransitions.length, 1);

    final scaleTransition1 = scaleTransitions.first;
    final scale1 = scaleTransition1.scale.value;

    // Advance time by 1s (half of duration)
    await tester.pump(const Duration(seconds: 1));

    final scaleTransition2 = tester.widgetList<ScaleTransition>(scaleFinder).first;
    final scale2 = scaleTransition2.scale.value;

    // Scale should have changed
    expect(scale1, isNot(scale2));
  });
}
