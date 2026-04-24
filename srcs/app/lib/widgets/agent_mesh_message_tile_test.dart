import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/agent_mesh_message_tile.dart';

void main() {
  testWidgets('AgentMeshMessageTile renders sender, message, and timestamp', (WidgetTester tester) async {
    final timestamp = DateTime(2026, 4, 23, 21, 30);
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AgentMeshMessageTile(
            sender: 'Agent Smith',
            message: 'Hello, World!',
            timestamp: timestamp,
          ),
        ),
      ),
    );

    // Fast forward animation
    await tester.pumpAndSettle();

    expect(find.text('Agent Smith'), findsOneWidget);
    expect(find.text('Hello, World!'), findsOneWidget);
    expect(find.text('21:30'), findsOneWidget);
  });

  testWidgets('AgentMeshMessageTile applies glassmorphism effects', (WidgetTester tester) async {
    final timestamp = DateTime.now();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AgentMeshMessageTile(
            sender: 'Agent Smith',
            message: 'Hello, World!',
            timestamp: timestamp,
          ),
        ),
      ),
    );

    await tester.pumpAndSettle();

    final backdropFinder = find.byType(BackdropFilter);
    expect(backdropFinder, findsOneWidget);

    final backdropFilter = tester.widget<BackdropFilter>(backdropFinder);
    expect(backdropFilter.filter.toString().contains('blur'), isTrue);
  });

  testWidgets('AgentMeshMessageTile animates on entry', (WidgetTester tester) async {
    final timestamp = DateTime.now();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AgentMeshMessageTile(
            sender: 'Agent Smith',
            message: 'Hello, World!',
            timestamp: timestamp,
          ),
        ),
      ),
    );

    // Initially scale should be at 0.95 (from animation tween begin)
    final scaleFinder = find.byType(ScaleTransition);
    expect(scaleFinder, findsWidgets);

    final scaleTransition = tester.widgetList<ScaleTransition>(scaleFinder).first;
    expect(scaleTransition.scale.value, closeTo(0.95, 0.01));

    // Pump to finish animation
    await tester.pumpAndSettle();

    // Now scale should be 1.0
    expect(scaleTransition.scale.value, 1.0);
  });
}
