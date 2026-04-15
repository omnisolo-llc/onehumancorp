import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/swarm_observability_dashboard.dart';

class MockMeshClient implements MeshClient {
  final _controller = StreamController<String>();
  @override
  Stream<String> get messageStream => _controller.stream;
  void addMessage(String msg) => _controller.add(msg);
  void close() => _controller.close();
}

void main() {
  testWidgets('SwarmObservabilityDashboard renders messages', (WidgetTester tester) async {
    final mockClient = MockMeshClient();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SwarmObservabilityDashboard(client: mockClient),
        ),
      ),
    );
    mockClient.addMessage('Agent Event 1');
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 100)); // allow animation to step
    expect(find.text('Agent Event 1'), findsOneWidget);
    mockClient.close();
  });
}
