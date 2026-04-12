import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('TaskListScreen renders correctly with tasks', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: TaskListScreen(),
        ),
      ),
    );

    expect(find.text('Shared Task List'), findsOneWidget);
    expect(find.text('Analyze User Feedback'), findsOneWidget);
    expect(find.text('Draft Feature Specification'), findsOneWidget);
    expect(find.text('Implement UI Component'), findsOneWidget);
    expect(find.byType(GlassCard), findsNWidgets(3));
  });
}
