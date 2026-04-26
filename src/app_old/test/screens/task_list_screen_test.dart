import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/models/shared_task.dart';

void main() {
  testWidgets('TaskListScreen renders properly and handles parent tasks', (WidgetTester tester) async {
    // Override the provider to return mocked tasks
    final tasks = [
      SharedTask(
        id: '1',
        title: 'Task 1',
        status: 'PENDING',
        parentTaskId: 'parent-123',
        workflowState: '{"step": "DECOMPOSING"}',
      ),
    ];

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          sharedTasksProvider.overrideWith((ref) => tasks),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: TaskListScreen(),
          ),
        ),
      ),
    );

    // Initial state might be loading or the immediate data
    await tester.pumpAndSettle();

    expect(find.text('Shared Task List'), findsOneWidget);
    expect(find.text('Task 1'), findsOneWidget);
    expect(find.text('Parent Task: parent-123'), findsOneWidget);
    expect(find.text('Workflow State: {"step": "DECOMPOSING"}'), findsOneWidget);
  });
}
