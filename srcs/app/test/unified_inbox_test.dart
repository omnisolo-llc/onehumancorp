import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/providers/inbox_provider.dart';
import 'package:app/screens/unified_inbox_screen.dart';

// We bypass the global setup and directly load the target screen to avoid state leakage and ProviderScope overrides on the full app
Future<void> launchInboxDirectly(WidgetTester tester, InboxState initialState) async {
  await tester.pumpWidget(
    ProviderScope(
      overrides: [
        inboxProvider.overrideWith((ref) => InboxNotifier()..state = initialState)
      ],
      child: const MaterialApp(home: UnifiedInboxScreen()),
    ),
  );
}

void main() {
  testWidgets('1. Unified Inbox Navigation E2E test', (WidgetTester tester) async {
    await launchInboxDirectly(tester, InboxState());
    expect(find.text("Unified Inbox"), findsOneWidget);
    expect(find.text("Connect Platforms"), findsOneWidget);
  });

  testWidgets('2. Unified Connection Hides CTA E2E test', (WidgetTester tester) async {
    await launchInboxDirectly(tester, InboxState());

    await tester.tap(find.byKey(const Key('connectInstagramBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byKey(const Key('connectWhatsappBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    // The Connect Platforms section should be gone
    expect(find.text("Connect Platforms"), findsNothing);
  });

  testWidgets('3. Unified Inbox Reply E2E test', (WidgetTester tester) async {
    await launchInboxDirectly(tester, InboxState(
      instagramConnected: true,
      messages: [
        InboxMessage(
            platform: "Instagram",
            sender: "real_customer",
            message: "Can I order 5 cupcakes for tomorrow?",
            time: "2m ago",
            isMe: false,
        )
      ]
    ));

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
