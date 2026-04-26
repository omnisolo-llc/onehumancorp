import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/help_chat_overlay.dart';
import 'package:ohc_app/widgets/walkthrough_overlay.dart';

void main() {
  testWidgets('HelpChatOverlay renders button', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: Scaffold(floatingActionButton: HelpChatOverlay())));
    expect(find.byType(FloatingActionButton), findsOneWidget);
    expect(find.byIcon(Icons.help_outline), findsOneWidget);
  });

  testWidgets('WalkthroughOverlay renders message', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: Scaffold(body: WalkthroughOverlay(message: 'Step 1'))));
    expect(find.text('Step 1'), findsOneWidget);
  });
}
