import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

void main() {
  setUpAll(() {
    HttpOverrides.global = null;
  });

  testWidgets('TaskListScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: TaskListScreen(),
      ),
    );

    await tester.pump(const Duration(seconds: 2));
    await tester.pumpAndSettle();

    expect(find.text('Shared Task List'), findsOneWidget);

    expect(find.byType(TaskGlassCard), findsWidgets);
    expect(find.text('Analyze Market Data'), findsOneWidget);
  });
}
