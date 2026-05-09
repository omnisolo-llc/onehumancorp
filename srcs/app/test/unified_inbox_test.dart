import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/screens/unified_inbox_screen.dart';
import 'package:app/providers/inbox_provider.dart';

void main() {
  testWidgets('1. Unified Inbox Navigation E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: UnifiedInboxScreen())));
    expect(find.text("Unified Inbox"), findsOneWidget);
    expect(find.text("Connect Platforms"), findsOneWidget);
  });

  testWidgets('2. Instagram Connection E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: UnifiedInboxScreen())));

    // Connect Instagram
    await tester.tap(find.byKey(const Key('connectInstagramBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    // Check if the messages are populated
    expect(find.text("maya_bakes"), findsOneWidget);
    expect(find.text("Do you do vegan cakes?"), findsOneWidget);
  });

  testWidgets('3. WhatsApp Connection E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: UnifiedInboxScreen())));

    // Connect WhatsApp
    await tester.tap(find.byKey(const Key('connectWhatsappBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    // Check if the messages are populated
    expect(find.text("+1 (555) 123-4567"), findsOneWidget);
    expect(find.text("Can I order 5 cupcakes for tomorrow?"), findsOneWidget);
  });

  testWidgets('4. Unified Connection Hides CTA E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: UnifiedInboxScreen())));

    await tester.tap(find.byKey(const Key('connectInstagramBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byKey(const Key('connectWhatsappBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.pump(const Duration(milliseconds: 500));

    // The Connect Platforms section should be gone
    expect(find.text("Connect Platforms"), findsNothing);
  });

  testWidgets('5. Unified Inbox Reply E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: UnifiedInboxScreen())));

    await tester.tap(find.byKey(const Key('connectInstagramBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    // Reply to the message
    await tester.enterText(find.byKey(const Key('replyTextField')), 'Yes, we do vegan cakes and cupcakes!');
    await tester.tap(find.byKey(const Key('sendReplyBtn')));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));

    // Verify our reply
    expect(find.text("Me"), findsOneWidget);
    expect(find.text("Yes, we do vegan cakes and cupcakes!"), findsOneWidget);
  });
}
