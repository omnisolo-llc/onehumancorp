import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

// Use relative import since ohc_web_app package might not be resolvable easily when run from srcs/app
import '../lib/widgets/swarm_observability_dashboard.dart';

void main() {
  testWidgets('SwarmObservabilityDashboard renders empty state correctly',
      (WidgetTester tester) async {
    final streamController = StreamController<MeshMessage>();

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityDashboard(
            messageStream: streamController.stream,
          ),
        ),
      ),
    );

    expect(find.text('Teammate Mesh Live Feed'), findsOneWidget);
    expect(find.text('Listening for swarm activity...'), findsOneWidget);

    streamController.close();
  });

  testWidgets('SwarmObservabilityDashboard renders incoming messages',
      (WidgetTester tester) async {
    final streamController = StreamController<MeshMessage>();

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityDashboard(
            messageStream: streamController.stream,
          ),
        ),
      ),
    );

    streamController.add(MeshMessage('AgentX', 'Task Initiated', DateTime(2026, 4, 14, 12, 0, 0)));
    await tester.pump(); // Process stream event
    await tester.pump(const Duration(milliseconds: 100)); // Process animations

    expect(find.text('AgentX'), findsOneWidget);
    expect(find.text('Task Initiated'), findsOneWidget);

    streamController.close();
  });
}
