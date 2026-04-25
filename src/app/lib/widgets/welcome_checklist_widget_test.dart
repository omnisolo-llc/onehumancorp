import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/welcome_checklist_widget.dart';

void main() {
  testWidgets('WelcomeChecklistWidget renders correctly and responds to interactions', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: WelcomeChecklistWidget(),
          ),
        ),
      ),
    );

    // Verify initial state
    expect(find.text('Welcome Checklist'), findsOneWidget);
    expect(find.text('Business live'), findsOneWidget);
    expect(find.text('Add 3 more products'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);
    expect(find.text('Share your link with a friend'), findsOneWidget);

    // The first one should be checked (value: true)
    var checkboxes = tester.widgetList<Checkbox>(find.byType(Checkbox));
    expect(checkboxes.elementAt(0).value, true);
    expect(checkboxes.elementAt(1).value, false);
    expect(checkboxes.elementAt(2).value, false);
    expect(checkboxes.elementAt(3).value, false);

    // Tap "Add 3 more products" checkbox (at index 1)
    await tester.tap(find.byType(Checkbox).at(1));
    await tester.pumpAndSettle();

    checkboxes = tester.widgetList<Checkbox>(find.byType(Checkbox));
    expect(checkboxes.elementAt(1).value, true);

    // Tap "Connect Instagram" checkbox (at index 2)
    await tester.tap(find.byType(Checkbox).at(2));
    await tester.pumpAndSettle();

    checkboxes = tester.widgetList<Checkbox>(find.byType(Checkbox));
    expect(checkboxes.elementAt(2).value, true);

    // Tap "Share your link with a friend" checkbox (at index 3)
    await tester.tap(find.byType(Checkbox).at(3));
    await tester.pumpAndSettle();

    checkboxes = tester.widgetList<Checkbox>(find.byType(Checkbox));
    expect(checkboxes.elementAt(3).value, true);
    expect(find.text('Link copied to clipboard!'), findsOneWidget);
  });
}
