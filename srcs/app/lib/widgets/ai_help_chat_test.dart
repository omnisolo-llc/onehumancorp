import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/ai_help_chat.dart';

void main() {
  testWidgets('AiHelpChat button toggles chat overlay', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SizedBox(
             width: 800,
             height: 600,
             child: Stack(
               children: [AiHelpChat()],
             ),
          ),
        ),
      ),
    );

    // Initial state: Chat should be closed
    expect(find.text('AI Support Agent'), findsNothing);

    // Open chat
    await tester.tap(find.byType(FloatingActionButton));
    await tester.pumpAndSettle();

    // Verify chat is open
    expect(find.text('AI Support Agent'), findsOneWidget);
    expect(find.text('Hi! I\'m your AI Support Agent. What do you need help with today?'), findsOneWidget);

    // Close chat via top right icon
    await tester.tap(find.byIcon(Icons.close).first);
    await tester.pumpAndSettle();

    // Verify chat is closed
    expect(find.text('AI Support Agent'), findsNothing);
  });
}
