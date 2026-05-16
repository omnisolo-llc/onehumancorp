import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/help/help_center_screen.dart';
import 'package:app/widgets/contextual_tooltip.dart';
import 'package:app/screens/help/ai_help_chat_screen.dart';
import 'package:app/main.dart'; // for GlassContainer

void main() {
  testWidgets('HelpCenterScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: HelpCenterScreen(),
        ),
      ),
    );

    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('How can we help?'), findsOneWidget);
    expect(find.text('Topics'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('Video Tutorials'), findsOneWidget);
    expect(find.text('Release Notes'), findsOneWidget);
    expect(find.text('API Reference'), findsOneWidget);
  });

  testWidgets('AiHelpChatScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: AiHelpChatScreen(),
        ),
      ),
    );

    expect(find.text('Ask Anything'), findsOneWidget);
    expect(find.text('Hi! I am your Support Agent. How can I help you with OneHumanCorp today?'), findsOneWidget);

    // Simulate sending a message
    await tester.enterText(find.byType(TextField), 'How do I add a product?');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pump();

    expect(find.text('How do I add a product?'), findsOneWidget);
    expect(find.text('This is a simulated AI response. Please visit the Help Center to read the full article.'), findsOneWidget);
  });

  testWidgets('ContextualTooltip renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: Center(
              child: ContextualTooltip(
                tooltipKey: 'industryDropdown',
                child: Text('Hover Me'),
              ),
            ),
          ),
        ),
      ),
    );

    expect(find.text('Hover Me'), findsOneWidget);
    expect(find.byType(Tooltip), findsOneWidget);
  });
}
