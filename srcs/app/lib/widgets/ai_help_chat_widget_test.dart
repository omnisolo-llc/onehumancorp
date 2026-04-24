import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/ai_help_chat_widget.dart';

void main() {
  testWidgets('AiHelpChatWidget renders FAB', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: Stack(
            children: [
              AiHelpChatWidget(),
            ],
          ),
        ),
      ),
    );

    expect(find.text('Ask AI Support'), findsOneWidget);
    expect(find.byIcon(Icons.help_outline), findsOneWidget);
  });

  testWidgets('AiHelpChatWidget opens bottom sheet on tap', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: Stack(
            children: [
              AiHelpChatWidget(),
            ],
          ),
        ),
      ),
    );

    await tester.tap(find.text('Ask AI Support'));
    await tester.pumpAndSettle(); // Wait for bottom sheet to open

    expect(find.text('AI Support Agent'), findsOneWidget);
    expect(find.textContaining('Hi there! I am your AI Support Agent'), findsOneWidget);
  });

  testWidgets('AiHelpChatWidget close button closes bottom sheet', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: Stack(
            children: [
              AiHelpChatWidget(),
            ],
          ),
        ),
      ),
    );

    await tester.tap(find.text('Ask AI Support'));
    await tester.pumpAndSettle();

    expect(find.text('AI Support Agent'), findsOneWidget);

    await tester.tap(find.byIcon(Icons.close));
    await tester.pumpAndSettle(); // Wait for bottom sheet to close

    expect(find.text('AI Support Agent'), findsNothing);
  });

  testWidgets('AiHelpChatWidget send button is disabled', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: Stack(
            children: [
              AiHelpChatWidget(),
            ],
          ),
        ),
      ),
    );

    await tester.tap(find.text('Ask AI Support'));
    await tester.pumpAndSettle();

    final sendButtonFinder = find.byIcon(Icons.send);
    expect(sendButtonFinder, findsOneWidget);

    final iconButton = tester.widget<IconButton>(find.ancestor(
      of: sendButtonFinder,
      matching: find.byType(IconButton),
    ));

    expect(iconButton.onPressed, isNull);
  });
}
