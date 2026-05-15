import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/main.dart';
import 'package:app/screens/unified_inbox_screen.dart';

void main() {
  testWidgets('Action Center UI and Optimistic Updates test', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: Scaffold(body: UnifiedInboxScreen()))));
    await tester.pumpAndSettle();

    // Verify Action Center header
    expect(find.text('Action Center'), findsOneWidget);

    // Verify initial mock tasks
    expect(find.text('Customer Reply Drafted'), findsOneWidget);
    expect(find.text('Quote Generated'), findsOneWidget);

    // Approve the first action
    final approveBtns = find.widgetWithText(ElevatedButton, 'Approve');
    expect(approveBtns, findsNWidgets(2));

    // Tap the first approve button ensuring it is visible
    await tester.ensureVisible(approveBtns.first);
    await tester.tap(approveBtns.first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Optimistic update should have removed the first task
    expect(find.text('Customer Reply Drafted'), findsNothing);
    expect(find.text('Quote Generated'), findsOneWidget);
    expect(find.widgetWithText(ElevatedButton, 'Approve'), findsOneWidget);

    // Reject the second action
    final rejectBtn = find.widgetWithText(TextButton, 'Reject').first;
    await tester.ensureVisible(rejectBtn);
    await tester.tap(rejectBtn, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Optimistic update should have removed the second task
    expect(find.text('Quote Generated'), findsNothing);
    expect(find.text('Action Center'), findsNothing);
  });
}
