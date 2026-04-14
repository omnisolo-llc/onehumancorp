import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_web_app/widgets/agent_message_state_animation.dart';

void main() {
  testWidgets('AgentMessageStateAnimation renders idle state correctly',
      (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentMessageStateAnimation(
            state: MessageState.idle,
            child: Text('Hello'),
          ),
        ),
      ),
    );

    expect(find.text('Hello'), findsOneWidget);
  });

  testWidgets('AgentMessageStateAnimation transitions to thinking state',
      (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentMessageStateAnimation(
            state: MessageState.thinking,
            child: Text('Thinking...'),
          ),
        ),
      ),
    );

    expect(find.text('Thinking...'), findsOneWidget);
    // Pump frames to ensure animation runs without throwing
    await tester.pump(const Duration(milliseconds: 300));
    await tester.pump(const Duration(milliseconds: 300));
  });

  testWidgets('AgentMessageStateAnimation handles state changes',
      (WidgetTester tester) async {
    Widget buildWidget(MessageState state) {
      return MaterialApp(
        home: Scaffold(
          body: AgentMessageStateAnimation(
            state: state,
            child: const Text('Dynamic'),
          ),
        ),
      );
    }

    await tester.pumpWidget(buildWidget(MessageState.idle));
    expect(find.text('Dynamic'), findsOneWidget);

    await tester.pumpWidget(buildWidget(MessageState.sending));
    await tester.pump(const Duration(milliseconds: 300));

    await tester.pumpWidget(buildWidget(MessageState.delivered));
    await tester.pumpAndSettle();
  });
}
