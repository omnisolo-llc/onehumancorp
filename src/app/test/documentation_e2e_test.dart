import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/widgets/ai_help_chat_widget.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

void main() {
  group('Documentation Features Tests', () {
    testWidgets('RegisteredTooltip resolves tooltip correctly from registry', (tester) async {
      TooltipRegistry().register('test_key', 'This is a helpful plain language description.');

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Center(
              child: RegisteredTooltip(
                tooltipKey: 'test_key',
                child: const Text('Hover Me'),
              ),
            ),
          ),
        ),
      );

      final tooltipFinder = find.byType(Tooltip);
      expect(tooltipFinder, findsOneWidget);

      final Tooltip tooltipWidget = tester.widget<Tooltip>(tooltipFinder);
      expect(tooltipWidget.message, 'This is a helpful plain language description.');
    });

    testWidgets('HelpCenterScreen renders topics and video sections', (tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: HelpCenterScreen(),
        ),
      );

      // Verify header and sections exist
      expect(find.text('How can we help you grow today?'), findsOneWidget);
      expect(find.text('Explore Topics'), findsOneWidget);
      expect(find.text('Video Tutorials'), findsOneWidget);

      // Verify some topics exist
      expect(find.text('Getting Started'), findsOneWidget);
      expect(find.text('Account & Billing'), findsOneWidget);

      // Verify video placeholders exist
      expect(find.text('Accepting your first payment'), findsOneWidget);
    });

    testWidgets('AiHelpChatWidget opens and accepts chat', (tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            floatingActionButton: AiHelpChatWidget(),
          ),
        ),
      );

      // Initial state is closed (FAB)
      final fabFinder = find.byType(FloatingActionButton);
      expect(fabFinder, findsOneWidget);
      expect(find.text('Ask AI Help'), findsOneWidget);

      // Tap to open chat
      await tester.tap(fabFinder);
      await tester.pumpAndSettle();

      // Verify chat UI appears
      expect(find.text('Help Agent'), findsOneWidget);
      final textFieldFinder = find.byType(TextField);
      expect(textFieldFinder, findsOneWidget);

      // Submit a query
      await tester.enterText(textFieldFinder, 'How do I add a product?');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      // Wait for chat to render message and bot reply
      expect(find.text('How do I add a product?'), findsOneWidget);
      expect(find.textContaining('Please refer to the Help Center'), findsOneWidget);
    });
  });
}
