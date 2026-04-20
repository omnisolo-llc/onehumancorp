import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/agent_task_progress.dart';

void main() {
  testWidgets('AgentTaskProgressWidget renders correctly when not working', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Analyzing Data',
            progress: 0.5,
            isWorking: false,
          ),
        ),
      ),
    );

    expect(find.text('Analyzing Data'), findsOneWidget);

    final progressBarFinder = find.byType(LinearProgressIndicator);
    expect(progressBarFinder, findsOneWidget);

    final LinearProgressIndicator progressBar = tester.widget(progressBarFinder);
    expect(progressBar.value, 0.5);

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));
  });

  testWidgets('AgentTaskProgressWidget animation controller runs when working', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Analyzing Data',
            progress: 0.5,
            isWorking: true,
          ),
        ),
      ),
    );

    expect(find.text('Analyzing Data'), findsOneWidget);

    // Pump to progress the animation
    await tester.pump(const Duration(milliseconds: 500));

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    // When half way through, value should be different than start (0.03)
    expect(decoration.color, isNot(equals(const Color.fromRGBO(255, 255, 255, 0.03))));

    // We cannot use pumpAndSettle because the animation repeats forever.
    // So we just pump one more frame to ensure no errors happen.
    await tester.pump();
  });
}
