import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'dart:ui';

void main() {
  testWidgets('TaskListScreen renders TaskGlassCards with Glassmorphism', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: TaskListScreen()));

    expect(find.byType(TaskGlassCard), findsWidgets);
    expect(find.text('Implement Login Screen'), findsOneWidget);
    expect(find.text('Setup CI/CD Pipeline'), findsOneWidget);

    final backdropFilterFinder = find.descendant(
      of: find.byType(TaskGlassCard),
      matching: find.byType(BackdropFilter),
    );

    expect(backdropFilterFinder, findsWidgets);
  });
}
