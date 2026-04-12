import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

void main() {
  testWidgets('TaskListScreen renders TaskGlassCards', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: TaskListScreen(),
      ),
    );

    expect(find.text('Shared Task List'), findsOneWidget);
    expect(find.text('Implement Core UI'), findsOneWidget);
    expect(find.text('Optimize Database'), findsOneWidget);
    expect(find.text('Write Docs'), findsOneWidget);
  });
}
