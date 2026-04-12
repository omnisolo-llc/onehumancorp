import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

void main() {
  testWidgets('TaskListScreen renders correctly with tasks', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: TaskListScreen(),
        ),
      ),
    );

    // Ensure the list is rendered and scrollable if items are off-screen
    await tester.pumpAndSettle();

    // Verify that the title is present.
    expect(find.text('Shared Task List'), findsOneWidget);
    expect(find.text('Swarm Tasks'), findsOneWidget);

    // Verify that tasks are rendered.
    expect(find.text('Train Nova LLM'), findsOneWidget);
    expect(find.text('Nova'), findsOneWidget);

    expect(find.text('Deploy Kubernetes Cluster'), findsOneWidget);
    expect(find.text('Implementer'), findsOneWidget);

    // For items that might be off-screen in the List, scroll to them
    final listFinder = find.byType(Scrollable);
    final itemFinder = find.text('Design AutoDream');

    await tester.scrollUntilVisible(
      itemFinder,
      500.0,
      scrollable: listFinder,
    );
    expect(itemFinder, findsOneWidget);
    expect(find.text('Palette'), findsOneWidget);

    // Verify Glassmorphism elements exist
    expect(find.byType(TaskGlassCard), findsWidgets);
  });
}