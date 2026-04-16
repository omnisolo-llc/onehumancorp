import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/vector_memory_visualizer.dart';

void main() {
  testWidgets('VectorMemoryVisualizerWidget renders correctly with OHC tokens', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: VectorMemoryVisualizerWidget(),
        ),
      ),
    );

    expect(find.text('AutoDream Consolidation'), findsOneWidget);
    expect(find.text('pgvector dimension: 1536'), findsOneWidget);

    final containerFinder = find.descendant(
      of: find.byType(BackdropFilter),
      matching: find.byType(Container).first,
    );

    final container = tester.widget<Container>(containerFinder);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));

    // Pump to let the animation tick
    await tester.pump(const Duration(milliseconds: 500));
  });
}
