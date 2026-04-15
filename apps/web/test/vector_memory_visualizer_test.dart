import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/vector_memory_visualizer.dart';

void main() {
  testWidgets('VectorMemoryVisualizerWidget renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: VectorMemoryVisualizerWidget(
            vector: [0.1, -0.5, 0.8, -0.2],
            content: 'Agent learned to write tests.',
          ),
        ),
      ),
    );

    expect(find.text('Agent learned to write tests.'), findsOneWidget);

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));

    // Check for nodes
    expect(find.byType(Wrap), findsOneWidget);

    await tester.pump(const Duration(milliseconds: 500));
  });
}
