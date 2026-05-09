import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/help/ai_help_chat_screen.dart';

void main() {
  testWidgets('E2E Audit - Verify No Mock/Stub Delays in AiHelpChatScreen', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: AiHelpChatScreen())));

    expect(find.text('Ask Anything'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Test message');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pump();

    // Verify simulated AI string is removed and we show the error since tests don't have mock backend running
    await tester.pump(const Duration(seconds: 1)); // wait for future
    expect(find.text('Error: Could not connect to backend.'), findsOneWidget);
    expect(find.text('This is a simulated AI response. Please visit the Help Center to read the full article.'), findsNothing);
  });
}
