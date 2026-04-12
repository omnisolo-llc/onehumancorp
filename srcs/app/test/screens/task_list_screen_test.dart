import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/task_model.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

void main() {
  group('TaskListScreen Widget', () {
    testWidgets('renders task details properly inside GlassCard', (WidgetTester tester) async {
      final task = Task(
        id: '1',
        title: 'Review the Pull Request',
        status: 'REVIEW',
        assignedAgent: 'Alpha',
        dependencies: ['test-dep'],
      );

      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TaskGlassCard(task: task),
            ),
          ),
        ),
      );

      expect(find.text('Review the Pull Request'), findsOneWidget);
      expect(find.text('REVIEW'), findsOneWidget);
      expect(find.text('Assigned Agent: Alpha'), findsOneWidget);
      expect(find.text('Dependencies:'), findsOneWidget);
      expect(find.text('test-dep'), findsOneWidget);
    });
  });
}
