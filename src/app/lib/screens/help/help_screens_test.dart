import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:ohc_app/screens/help/help_center_screen.dart';
import 'package:ohc_app/screens/help/help_article_screen.dart';
import 'package:ohc_app/screens/help/changelog_screen.dart';
import 'package:ohc_app/screens/help/api_docs_screen.dart';
import 'package:ohc_app/widgets/ai_help_chat_widget.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

void main() {
  testWidgets('HelpCenterScreen renders correctly', (WidgetTester tester) async {
    // Set a large screen size to avoid scrolling issues
    tester.view.physicalSize = const Size(1080, 2400);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(const MaterialApp(home: HelpCenterScreen()));
    await tester.pumpAndSettle();

    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Getting Started'), findsOneWidget);
    expect(find.text('My Store'), findsOneWidget);

    // reset view
    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });



  testWidgets('HelpArticleScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: HelpArticleScreen(articleId: 'getting-started')));
    expect(find.text('Getting Started'), findsNWidgets(2)); // AppBar and Title
  });

  testWidgets('ChangelogScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: ChangelogScreen()));
    expect(find.text('What\'s New'), findsOneWidget);
  });

  testWidgets('ApiDocsScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: ApiDocsScreen()));
    expect(find.text('API Reference'), findsOneWidget);
  });

  testWidgets('AiHelpChatWidget renders button', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: Scaffold(body: AiHelpChatWidget())));
    expect(find.text('Ask AI Support'), findsOneWidget);
  });

  test('TooltipRegistry returns valid strings', () {
    expect(TooltipRegistry.get('dashboard_refresh'), isNotEmpty);
    expect(TooltipRegistry.get('missing_key_123'), contains('missing_key_123'));
  });
}
