import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/widgets/agent_activity_widget.dart';
import 'package:ohc_app/services/centrifuge_service.dart';

class MockCentrifugeService extends Mock implements CentrifugeService {}

void main() {
  testWidgets('AgentActivityWidget renders and displays messages', (
    WidgetTester tester,
  ) async {
    final mockService = MockCentrifugeService();

    when(() => mockService.subscribeRaw('mesh:tasks')).thenAnswer(
      (_) => Stream.fromIterable([
        {'agent_id': 'Agent 1', 'action': 'Thinking', 'status': 'Working'},
        {'agent_id': 'Agent 2', 'action': 'Coding', 'status': 'Done'},
      ]),
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [centrifugeServiceProvider.overrideWithValue(mockService)],
        child: const MaterialApp(home: Scaffold(body: AgentActivityWidget())),
      ),
    );

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.text('Agent Activity Feed'), findsOneWidget);

    expect(find.text('Agent 1'), findsOneWidget);
    expect(find.text('Thinking'), findsOneWidget);

    expect(find.text('Agent 2'), findsOneWidget);
    expect(find.text('Coding'), findsOneWidget);
  });
}
