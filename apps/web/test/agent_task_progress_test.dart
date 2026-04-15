import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_web_app/widgets/agent_task_progress.dart';

void main() {
  testWidgets('AgentTaskProgressWidget renders correctly with default inProgress status', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Analyzing Data',
            progress: 0.5,
          ),
        ),
      ),
    );

    expect(find.text('Analyzing Data'), findsOneWidget);
    expect(find.text('In Progress'), findsOneWidget);
    expect(find.byIcon(Icons.hourglass_empty), findsOneWidget);

    final progressBarFinder = find.byType(LinearProgressIndicator);
    expect(progressBarFinder, findsOneWidget);

    final LinearProgressIndicator progressBar = tester.widget(progressBarFinder);
    expect(progressBar.value, 0.5);
    expect(progressBar.valueColor!.value, Colors.blueAccent);
  });

  testWidgets('AgentTaskProgressWidget renders correctly with completed status', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Analyzing Data',
            progress: 1.0,
            status: TaskStatus.completed,
          ),
        ),
      ),
    );

    expect(find.text('Analyzing Data'), findsOneWidget);
    expect(find.text('Completed'), findsOneWidget);
    expect(find.byIcon(Icons.check_circle_outline), findsOneWidget);

    final progressBarFinder = find.byType(LinearProgressIndicator);
    expect(progressBarFinder, findsOneWidget);

    final LinearProgressIndicator progressBar = tester.widget(progressBarFinder);
    expect(progressBar.value, 1.0);
    expect(progressBar.valueColor!.value, Colors.greenAccent);
  });

  testWidgets('AgentTaskProgressWidget renders correctly with failed status', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Analyzing Data',
            progress: 0.5,
            status: TaskStatus.failed,
          ),
        ),
      ),
    );

    expect(find.text('Analyzing Data'), findsOneWidget);
    expect(find.text('Failed'), findsOneWidget);
    expect(find.byIcon(Icons.error_outline), findsOneWidget);

    final progressBarFinder = find.byType(LinearProgressIndicator);
    expect(progressBarFinder, findsOneWidget);

    final LinearProgressIndicator progressBar = tester.widget(progressBarFinder);
    expect(progressBar.value, 0.5);
    expect(progressBar.valueColor!.value, Colors.redAccent);
  });
}
