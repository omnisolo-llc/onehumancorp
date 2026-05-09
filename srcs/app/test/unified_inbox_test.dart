import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/unified_inbox_screen.dart';

void main() {
  Widget createWidgetUnderTest() {
    return const ProviderScope(
      child: MaterialApp(
        home: UnifiedInboxScreen(),
      ),
    );
  }

  testWidgets('1. Unified Inbox Navigation E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    expect(find.text('Unified Inbox'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);
    expect(find.text('Connect WhatsApp'), findsOneWidget);
  });

  testWidgets('2. Instagram Connection E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    await tester.tap(find.text('Connect Instagram'));
    await tester.pumpAndSettle();

    // Verify UI reflects Instagram is connected (the connect button disappears)
    expect(find.text('Connect Instagram'), findsNothing);
  });

  testWidgets('3. WhatsApp Connection E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    await tester.tap(find.text('Connect WhatsApp'));
    await tester.pumpAndSettle();

    // Verify UI reflects WhatsApp is connected (the connect button disappears)
    expect(find.text('Connect WhatsApp'), findsNothing);
  });

  testWidgets('4. Unified Connection Hides CTA E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    await tester.tap(find.text('Connect Instagram'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Connect WhatsApp'));
    await tester.pumpAndSettle();

    expect(find.text('Connect Platforms'), findsNothing);
    expect(find.text('Connect Instagram'), findsNothing);
    expect(find.text('Connect WhatsApp'), findsNothing);
  });

  testWidgets('5. Unified Inbox Reply E2E test', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    // The messages array is initially empty. Need to connect to a platform first to see a message.
    await tester.tap(find.text('Connect Instagram'));
    await tester.pumpAndSettle();

    // Verify the initial message exists
    expect(find.text('Do you do vegan cakes?'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Yes it is!');
    await tester.tap(find.byIcon(Icons.send));
    await tester.pumpAndSettle();

    // Verify the reply appears in the UI
    expect(find.text('Yes it is!'), findsOneWidget);
  });
}
