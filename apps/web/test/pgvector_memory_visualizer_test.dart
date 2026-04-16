import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/pgvector_memory_visualizer.dart';

void main() {
  testWidgets('PgvectorMemoryVisualizerWidget renders correctly with OHC tokens', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: PgvectorMemoryVisualizerWidget(),
        ),
      ),
    );

    expect(find.text('Pgvector Memory Visualizer'), findsOneWidget);

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump();
  });
}
