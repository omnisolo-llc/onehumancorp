import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/ai_help_chat_widget.dart';

void main() {
  testWidgets('AiHelpChatWidget opens and handles chat', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: Stack(
              children: [
                AiHelpChatWidget(),
              ],
            ),
          ),
        ),
      ),
    );

    // Initial state: Chat is closed, only FAB is visible.
    expect(find.byKey(const Key('ai_help_chat_button')), findsOneWidget);
    expect(find.text('Ask OHC Help'), findsNothing);

    // Tap to open
    await tester.tap(find.byKey(const Key('ai_help_chat_button')));
    await tester.pumpAndSettle();

    // Now chat UI is visible
    expect(find.text('Ask OHC Help'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);

    // Enter a message
    await tester.enterText(find.byType(TextField), 'Hello OHC Help');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pumpAndSettle();

    // Verify messages appear
    expect(find.text('Hello OHC Help'), findsOneWidget);
    expect(find.text("That's a great question! For more details on this topic, please visit our Help Center articles."), findsOneWidget);

    // Close chat using top-right close icon
    await tester.tap(find.byTooltip('Close chat'));
    await tester.pumpAndSettle();

    expect(find.text('Ask OHC Help'), findsNothing);
  });
}
