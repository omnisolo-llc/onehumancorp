import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/slide_to_approve.dart';

void main() {
  testWidgets('SlideToApprove renders text and button', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SlideToApprove(
            onApprove: () {},
            onReject: () {},
          ),
        ),
      ),
    );

    expect(find.text('Slide to Approve'), findsOneWidget);
    expect(find.text('Reject Request'), findsOneWidget);
  });

  testWidgets('SlideToApprove calls onReject on button tap', (WidgetTester tester) async {
    bool rejected = false;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SlideToApprove(
            onApprove: () {},
            onReject: () {
              rejected = true;
            },
          ),
        ),
      ),
    );

    await tester.tap(find.text('Reject Request'));
    await tester.pump();

    expect(rejected, isTrue);
  });

  testWidgets('SlideToApprove calls onApprove on full slide', (WidgetTester tester) async {
    bool approved = false;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SlideToApprove(
            onApprove: () {
              approved = true;
            },
            onReject: () {},
          ),
        ),
      ),
    );

    final gestureFinder = find.descendant(
      of: find.bySemanticsLabel('Slide to Approve'),
      matching: find.byType(GestureDetector),
    );
    expect(gestureFinder, findsOneWidget);

    // Drag the thumb from left to right
    final center = tester.getCenter(gestureFinder);
    final gesture = await tester.startGesture(center);
    // Max drag is 240. So we drag by at least 240.
    await gesture.moveBy(const Offset(300, 0));
    await gesture.up();
    await tester.pumpAndSettle();

    expect(approved, isTrue);
  });

  testWidgets('SlideToApprove does not allow drag when disabled', (WidgetTester tester) async {
    bool approved = false;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SlideToApprove(
            onApprove: () {
              approved = true;
            },
            onReject: () {},
            disabled: true,
          ),
        ),
      ),
    );

    final gestureFinder = find.descendant(
      of: find.bySemanticsLabel('Slide to Approve'),
      matching: find.byType(GestureDetector),
    );
    final center = tester.getCenter(gestureFinder);
    final gesture = await tester.startGesture(center);
    await gesture.moveBy(const Offset(300, 0));
    await gesture.up();
    await tester.pumpAndSettle();

    expect(approved, isFalse);
  });
}
