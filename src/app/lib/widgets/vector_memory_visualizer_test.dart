import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/vector_memory_visualizer.dart';

void main() {
  testWidgets('VectorMemoryVisualizerWidget renders title and dimension', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: VectorMemoryVisualizerWidget(),
        ),
      ),
    );

    expect(find.text('AutoDream Consolidation'), findsOneWidget);
    expect(find.text('pgvector dimension: 1536'), findsOneWidget);
  });

  testWidgets('VectorMemoryVisualizerWidget animates', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: VectorMemoryVisualizerWidget(),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final transformFinder = find.byType(Transform);
    expect(transformFinder, findsWidgets);

    final transform1 = tester.widgetList<Transform>(transformFinder).first;
    final matrix1 = transform1.transform;

    // Advance time by 500ms
    await tester.pump(const Duration(milliseconds: 500));

    final transform2 = tester.widgetList<Transform>(transformFinder).first;
    final matrix2 = transform2.transform;

    // Transform matrix should have changed
    expect(matrix1, isNot(matrix2));
  });
}
