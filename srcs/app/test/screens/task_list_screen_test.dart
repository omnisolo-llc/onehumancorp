import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

void main() {
  testWidgets('TaskListScreen renders properly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: TaskListScreen(),
          ),
        ),
      ),
    );

    expect(find.text('Shared Task List'), findsOneWidget);
  });
}
