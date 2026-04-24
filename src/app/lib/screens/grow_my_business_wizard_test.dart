import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';

void main() {
  testWidgets('GrowMyBusinessWizardScreen transitions', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(
      child: MaterialApp(
        home: GrowMyBusinessWizardScreen(),
      ),
    ));

    // Verify step 0
    expect(find.text('Next steps to grow your business'), findsOneWidget);
    expect(find.text('Start'), findsOneWidget);

    // Tap "Start" and advance to step 1
    await tester.tap(find.text('Start'));
    await tester.pumpAndSettle();

    expect(find.text('Add 5 more products'), findsOneWidget);
    expect(find.text('Do it now'), findsOneWidget);
    expect(find.text('I\'ll do it later'), findsOneWidget);

    // Tap "I'll do it later" and advance to step 2
    await tester.tap(find.text('I\'ll do it later'));
    await tester.pumpAndSettle();

    expect(find.text('Connect Instagram'), findsOneWidget);

    // Tap "Do it now" and advance to step 3
    await tester.tap(find.text('Do it now'));
    await tester.pumpAndSettle();

    expect(find.text('Run your first email campaign'), findsOneWidget);
    expect(find.text('Finish'), findsOneWidget);
  });
}
