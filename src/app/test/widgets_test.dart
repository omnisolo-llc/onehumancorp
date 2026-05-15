import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:app/widgets/swarm_velocity_widget.dart';
import 'package:app/widgets/vector_memory_visualizer.dart';
import 'package:app/screens/dashboard_screen.dart';
import 'package:app/screens/swarm_memory_screen.dart';
import 'package:app/main.dart';

void main() {
  group('SwarmVelocityWidget Tests', () {
    testWidgets('renders task completion rate and latency', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: SwarmVelocityWidget(
              taskCompletionRate: 85.5,
              latencyMs: 120.0,
            ),
          ),
        ),
      );

      expect(find.text('Swarm Velocity'), findsOneWidget);
      expect(find.text('Task Rate'), findsOneWidget);
      expect(find.text('85.5 /min'), findsOneWidget);
      expect(find.text('Latency'), findsOneWidget);
      expect(find.text('120 ms'), findsOneWidget);
    });
  });

  group('VectorMemoryVisualizerWidget Tests', () {
    testWidgets('renders correctly and handles pulsing', (WidgetTester tester) async {
      final vectorState = List.generate(150, (index) => 0.5);

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: VectorMemoryVisualizerWidget(
              vectorState: vectorState,
              isPulsing: false,
            ),
          ),
        ),
      );

      // Verify custom paint renders
      expect(find.byType(CustomPaint), findsWidgets);

      // Update to pulsing state
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: VectorMemoryVisualizerWidget(
              vectorState: vectorState,
              isPulsing: true,
            ),
          ),
        ),
      );

      // Allow animations to start without using pumpAndSettle due to repeating controller
      await tester.pump(const Duration(milliseconds: 500));
      expect(find.byType(CustomPaint), findsWidgets);
    });

    testWidgets('renders gracefully with empty state', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: VectorMemoryVisualizerWidget(
              vectorState: [],
            ),
          ),
        ),
      );

      expect(find.byType(CustomPaint), findsWidgets);
    });
  });

  group('Screen Tests', () {
    testWidgets('DashboardScreen renders and navigates', (WidgetTester tester) async {
      await tester.pumpWidget(const MyApp());

      expect(find.text('OHC Business Dashboard'), findsOneWidget);
      expect(find.byType(SwarmVelocityWidget), findsOneWidget);

      final viewMemoryBtn = find.byKey(const Key('view_memory_btn'));
      expect(viewMemoryBtn, findsOneWidget);

      await tester.tap(viewMemoryBtn);
      await tester.pump();
      await tester.pump(const Duration(milliseconds: 500)); // wait for nav transition

      expect(find.text('Swarm Memory State'), findsOneWidget);
      expect(find.byType(VectorMemoryVisualizerWidget), findsOneWidget);
    });

    testWidgets('SwarmMemoryScreen handles broadcast event', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: SwarmMemoryScreen(),
        ),
      );

      final broadcastBtn = find.byKey(const Key('broadcast_event_btn'));
      expect(broadcastBtn, findsOneWidget);

      await tester.tap(broadcastBtn);
      await tester.pump();
      // wait for pulse to settle, but don't pumpAndSettle because of repeating animation
      await tester.pump(const Duration(milliseconds: 500));

      expect(find.byType(VectorMemoryVisualizerWidget), findsOneWidget);
    });
  });
}
