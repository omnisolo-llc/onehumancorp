import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:flutter/material.dart';

void main() {
  testWidgets('TaskListScreen renders mock task', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: TaskListScreen()));
    expect(find.text('Implement Shared Task List'), findsOneWidget);
    expect(find.text('Status: IN_PROGRESS | Agent: Palette'), findsOneWidget);
  });
}
