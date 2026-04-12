import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('TaskListScreen renders title and tasks with GlassCard', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: TaskListScreen(),
        ),
      ),
    );

    // Initial loading state
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Wait for the mock provider to resolve
    await tester.pump(const Duration(seconds: 1));
    await tester.pumpAndSettle();

    // Verify title
    expect(find.text('Shared Task List'), findsOneWidget);

    // Verify tasks are rendered
    expect(find.text('Analyze market data'), findsOneWidget);
    expect(find.text('Generate UI components'), findsOneWidget);

    // Verify GlassCards are used
    expect(find.byType(GlassCard), findsWidgets);
  });
}
