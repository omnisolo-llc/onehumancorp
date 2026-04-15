import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/agent_task_progress.dart';

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
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.05));
  });

  testWidgets('AgentTaskProgressWidget renders correctly when working', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Processing',
            progress: 0.8,
            isWorking: true,
          ),
        ),
      ),
    );

    expect(find.text('Processing'), findsOneWidget);

    final progressBarFinder = find.byType(LinearProgressIndicator);
    expect(progressBarFinder, findsOneWidget);

    final LinearProgressIndicator progressBar = tester.widget(progressBarFinder);
    expect(progressBar.value, 0.8);

    expect(
      find.descendant(
        of: find.byType(AgentTaskProgressWidget),
        matching: find.byType(FadeTransition),
      ),
      findsOneWidget,
    );
  });
}
