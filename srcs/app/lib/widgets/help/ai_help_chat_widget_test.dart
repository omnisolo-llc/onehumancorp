import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/help/ai_help_chat_widget.dart';

void main() {
  testWidgets('AIHelpChatWidget opens and closes', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: Scaffold(body: AIHelpChatWidget())));

    expect(find.byType(FloatingActionButton), findsOneWidget);
    expect(find.text('AI Help Agent'), findsNothing);

    await tester.tap(find.byType(FloatingActionButton));
    await tester.pumpAndSettle();

    expect(find.text('AI Help Agent'), findsOneWidget);
    expect(find.text('Hi! I am your AI Help Agent. You can ask me anything about setting up your business on OHC.'), findsOneWidget);

    await tester.tap(find.byIcon(Icons.close).first);
    await tester.pumpAndSettle();

    expect(find.text('AI Help Agent'), findsNothing);
  });
}
