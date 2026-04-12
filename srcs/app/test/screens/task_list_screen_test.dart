import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/models/task.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('TaskListScreen renders correctly and shows data from provider', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          taskListProvider.overrideWith((ref) async => [
            const SharedTask(
              id: '1',
              title: 'Analyze Market Trends',
              assignedAgent: 'Researcher-1',
              status: 'IN_PROGRESS',
              dependencies: [],
            ),
            const SharedTask(
              id: '2',
              title: 'Update UI Components',
              assignedAgent: 'Palette-Lead',
              status: 'PENDING',
              dependencies: ['1'],
            ),
          ]),
        ],
        child: const MaterialApp(
          home: TaskListScreen(),
        ),
      ),
    );

    // Initial state might be loading, pump to let future complete
    await tester.pumpAndSettle();

    // Verify title
    expect(find.text('Shared Task List'), findsOneWidget);

    // Verify we have 2 TaskGlassCards
    expect(find.byType(TaskGlassCard), findsNWidgets(2));

    // Verify specific task texts
    expect(find.text('Analyze Market Trends'), findsOneWidget);
    expect(find.text('IN_PROGRESS'), findsOneWidget);
    expect(find.text('Assigned Agent: Researcher-1'), findsOneWidget);

    expect(find.text('Update UI Components'), findsOneWidget);
    expect(find.text('PENDING'), findsOneWidget);
    expect(find.text('Assigned Agent: Palette-Lead'), findsOneWidget);
    expect(find.text('Dependencies: 1'), findsOneWidget);

    // Verify it uses the GlassCard
    expect(find.byType(GlassCard), findsNWidgets(2));
  });

  testWidgets('TaskGlassCard renders correctly', (WidgetTester tester) async {
    const task = SharedTask(
      id: '3',
      title: 'Test Task',
      assignedAgent: 'Test Agent',
      status: 'COMPLETED',
      dependencies: ['1', '2'],
    );

    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: TaskGlassCard(task: task),
        ),
      ),
    );

    expect(find.text('Test Task'), findsOneWidget);
    expect(find.text('COMPLETED'), findsOneWidget);
    expect(find.text('Assigned Agent: Test Agent'), findsOneWidget);
    expect(find.text('Dependencies: 1, 2'), findsOneWidget);
  });
}
