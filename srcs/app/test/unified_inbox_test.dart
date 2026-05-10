import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';

// Helper function to quickly navigate to the inbox screen for tests that don't need the wizard flow
Future<void> navigateToInbox(WidgetTester tester) async {
  await tester.pumpWidget(const ProviderScope(child: OHCApp()));

  // 1. Welcome Screen
  final emailField = find.byKey(const Key('signupEmailField'));
  await tester.ensureVisible(emailField);
  await tester.enterText(emailField, 'test@test.com');
  await tester.enterText(find.byKey(const Key('signupPasswordField')), 'pw');
  final signupBtn = find.byKey(const Key('signupBtn'));
  await tester.ensureVisible(signupBtn);
  await tester.tap(signupBtn);
  await tester.pump(const Duration(milliseconds: 500));

  // 2. Business Profile Screen
  await tester.enterText(find.byKey(const Key('companyNameField')), 'Acme Corp');
  await tester.tap(find.byKey(const Key('industryDropdown')));
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.text('Technology').last);
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.byKey(const Key('sizeDropdown')));
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.text('11-50').last);
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 3. Goal Selection Screen
  await tester.tap(find.text('Build software'));
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.text('Support'));
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 4. External Integrations Screen
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 5. Deployment Preference Screen
  await tester.tap(find.text('Cloud'));
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 6. Administrator Account Screen
  await tester.enterText(find.byKey(const Key('adminNameField')), 'John Doe');
  await tester.enterText(find.byKey(const Key('adminEmailField')), 'john@acme.com');
  await tester.enterText(find.byKey(const Key('adminPasswordField')), 'securePassword123');
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 7. Template Selection Screen
  await tester.tap(find.text('Modern'));
  await tester.pump(const Duration(milliseconds: 100));
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 8. Product Screen
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 9. Domain Screen
  await tester.tap(find.byType(ElevatedButton).last);
  await tester.pump(const Duration(milliseconds: 500));

  // 10. Review & Launch Screen
  final launchBtn = find.byKey(const Key('launchAIBtn'));
  await tester.ensureVisible(launchBtn);
  await tester.pump(const Duration(milliseconds: 500));
  await tester.tap(launchBtn);

  // Wait for pulse animation and API call
  await tester.pump(const Duration(seconds: 2));

  // 11. Checklist Screen
  await tester.tap(find.text('Go to Dashboard'));
  await tester.pump(const Duration(milliseconds: 500));

  // Dashboard is visible now. Avoid pumpAndSettle due to pulse animation running.
  // Wait explicitly instead
  await tester.pump(const Duration(seconds: 1));

  // 7. Dashboard Screen
  final inboxBtn = find.byKey(const Key('inboxBtn'));
  await tester.dragUntilVisible(
    inboxBtn,
    find.byType(ListView).first,
    const Offset(0, -100),
  );
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
