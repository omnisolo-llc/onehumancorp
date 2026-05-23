import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/swarm_velocity_widget.dart';
import '../lib/widgets/vector_memory_visualizer.dart';

void main() {
  group('SwarmVelocityWidget Tests', () {
    testWidgets('renders correctly with given metrics', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: SwarmVelocityWidget(
              taskCompletionRate: 45.2,
              latencyMs: 120.0,
            ),
          ),
        ),
      );

      expect(find.text('Swarm Velocity'), findsOneWidget);
      expect(find.text('45.2/s'), findsOneWidget);
      expect(find.text('120 ms'), findsOneWidget);
    });
  });

  group('VectorMemoryVisualizerWidget Tests', () {
    testWidgets('renders vector memory state correctly', (WidgetTester tester) async {
      final dummyVector = List.generate(1536, (index) => (index % 100) / 100.0);

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: VectorMemoryVisualizerWidget(
              vectorState: dummyVector,
              isPulsing: false,
            ),
          ),
        ),
      );

      expect(find.text('Vector Memory State (1536-dim)'), findsOneWidget);
      expect(find.byType(CustomPaint), findsWidgets);
    });
  });
}
