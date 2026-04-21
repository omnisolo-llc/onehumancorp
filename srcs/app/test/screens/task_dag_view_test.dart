import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_dag_view.dart';

void main() {
  testWidgets('TaskDAGView renders properly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: TaskDAGView(),
          ),
        ),
      ),
    );

    expect(find.text('Shared Task List'), findsOneWidget);
  });
}
