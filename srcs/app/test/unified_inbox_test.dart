import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';

// Helper function to quickly navigate to the inbox screen for tests that don't need the wizard flow
Future<void> navigateToInbox(WidgetTester tester) async {
  await tester.pumpWidget(const ProviderScope(child: OHCApp()));

  // 1. Category Screen
  await tester.tap(find.text('Bake'));
  await tester.pump(const Duration(milliseconds: 500)); // Auto-advances

  // 2. Name Screen
  await tester.enterText(find.byKey(const Key('companyNameField')), 'Maya\'s Bakes');
  await tester.tap(find.text('Next'));

  // In Flutter tests, async state changes often need multiple pumps to propagate.
  await tester.pump();
  await tester.pump();
  await tester.pump();

  // Wait for the simulated API call (2 seconds)
  await tester.pump(const Duration(seconds: 2));
  await tester.pump(const Duration(milliseconds: 500));

  // 4. Dashboard Screen
  // In the minimal flow, Dashboard is currently shown on `currentStep == 3`.
  // Let's first ensure we are on the dashboard by looking for a text that should be there.
  await tester.ensureVisible(find.text("Welcome Checklist"));
  await tester.pump(const Duration(milliseconds: 500));

  final inboxBtn = find.byKey(const Key('inboxBtn'));
  expect(inboxBtn, findsOneWidget); // Make sure it exists before trying to scroll/tap
  await tester.ensureVisible(inboxBtn);
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(inboxBtn);
  await tester.pump(const Duration(milliseconds: 500));
  await tester.pump(const Duration(seconds: 1)); // Navigator transition
}

void main() {
  testWidgets('1. Unified Inbox Navigation E2E test', (WidgetTester tester) async {
    await navigateToInbox(tester);
    expect(find.text("Unified Inbox"), findsOneWidget);
    expect(find.text("Connect Platforms"), findsOneWidget);
  });

  testWidgets('2. Instagram Connection E2E test', (WidgetTester tester) async {
    await navigateToInbox(tester);

    // Connect Instagram
    await tester.tap(find.byKey(const Key('connectInstagramBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    // Check if the messages are populated
    expect(find.text("maya_bakes"), findsOneWidget);
    expect(find.text("Do you do vegan cakes?"), findsOneWidget);
  });

  testWidgets('3. WhatsApp Connection E2E test', (WidgetTester tester) async {
    await navigateToInbox(tester);

    // Connect WhatsApp
    await tester.tap(find.byKey(const Key('connectWhatsappBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    // Check if the messages are populated
    expect(find.text("+1 (555) 123-4567"), findsOneWidget);
    expect(find.text("Can I order 5 cupcakes for tomorrow?"), findsOneWidget);
  });

  testWidgets('4. Unified Connection Hides CTA E2E test', (WidgetTester tester) async {
    await navigateToInbox(tester);

    await tester.tap(find.byKey(const Key('connectInstagramBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.tap(find.byKey(const Key('connectWhatsappBtn')));
    await tester.pump(const Duration(milliseconds: 500));

    await tester.pump(const Duration(milliseconds: 500));

    // The Connect Platforms section should be gone
    expect(find.text("Connect Platforms"), findsNothing);
  });

  testWidgets('5. Unified Inbox Reply E2E test', (WidgetTester tester) async {
    await navigateToInbox(tester);

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
