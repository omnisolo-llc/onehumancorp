import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_web_app/screens/swarm_observability_dashboard_screen.dart';

class TestMeshBroker implements IMeshBroker {
  final _controller = StreamController<dynamic>.broadcast();

  @override
  Stream<dynamic> get stream => _controller.stream;

  @override
  void close() {
    _controller.close();
  }

  void pushMessage(String message) {
    _controller.add(message);
  }
}

void main() {
  testWidgets('SwarmObservabilityDashboardScreen renders elements correctly and updates from broker', (WidgetTester tester) async {
    final broker = TestMeshBroker();
    await tester.pumpWidget(MaterialApp(home: SwarmObservabilityDashboardScreen(broker: broker)));

    // Verify title
    expect(find.text('Swarm Observability'), findsOneWidget);
    expect(find.text('Active Swarm Mesh'), findsOneWidget);

    // Initial state before websocket receives data
    expect(find.text('Waiting for mesh telemetry...'), findsOneWidget);

    // Push message
    broker.pushMessage('{"agent_id": "Implementer", "action": "Building Dashboard", "status": "IN_PROGRESS"}');
    await tester.pumpAndSettle();

    // Verify list items updated
    expect(find.text('Waiting for mesh telemetry...'), findsNothing);
    expect(find.text('Implementer'), findsOneWidget);
    expect(find.text('Building Dashboard'), findsOneWidget);
    expect(find.text('IN_PROGRESS'), findsOneWidget);
  });
}
