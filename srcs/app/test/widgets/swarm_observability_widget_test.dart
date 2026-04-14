import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';
import 'package:ohc_app/services/centrifuge_service.dart';

class MockCentrifugeService extends CentrifugeService {
  MockCentrifugeService() : super(
    serverUrl: 'ws://localhost/connection/websocket',
    token: 'test_token',
    userId: 'test_user',
    userName: 'Test User',
  );

  @override
  Stream<dynamic> subscribeRaw(String channel) {
    return Stream.value({'agent_id': 'Test Agent', 'action': 'Test Action'});
  }
}

void main() {
  testWidgets('SwarmObservabilityWidget renders and animates', (WidgetTester tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          centrifugeServiceProvider.overrideWithValue(MockCentrifugeService()),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: SwarmObservabilityWidget(),
          ),
        ),
      ),
    );

    // Initial render
    expect(find.text('Teammate Mesh Live Feed'), findsOneWidget);

    // Allow stream to emit
    await tester.pump();

    // Allow animations to run
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Test Agent'), findsOneWidget);
    expect(find.text('Test Action'), findsOneWidget);
  });
}
