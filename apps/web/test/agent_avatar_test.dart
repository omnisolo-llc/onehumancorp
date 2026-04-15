import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/agent_avatar.dart';

void main() {
  testWidgets('AgentAvatar renders initial correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentAvatar(agentName: 'Jules', isWorking: false),
        ),
      ),
    );

    expect(find.text('J'), findsOneWidget);
  });

  testWidgets('AgentAvatar respects isWorking property changes', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentAvatar(agentName: 'Jules', isWorking: true),
        ),
      ),
    );
    expect(find.text('J'), findsOneWidget);
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentAvatar(agentName: 'Jules', isWorking: false),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('J'), findsOneWidget);
  });
}
