import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/pulse_animation.dart';

void main() {
  testWidgets('PulseAnimation renders child', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: PulseAnimation(
            child: Text('Target'),
          ),
        ),
      ),
    );

    expect(find.text('Target'), findsOneWidget);
  });

  testWidgets('PulseAnimation pulses', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: PulseAnimation(
            child: Text('Target'),
          ),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final scaleFinder = find.descendant(
      of: find.byType(PulseAnimation),
      matching: find.byType(ScaleTransition),
    );
    expect(scaleFinder, findsOneWidget);

    final scaleTransition1 = tester.widgetList<ScaleTransition>(scaleFinder).first;
    final scale1 = scaleTransition1.scale.value;

    // Advance time by 500ms (half of default duration 1000ms)
    await tester.pump(const Duration(milliseconds: 500));

    final scaleTransition2 = tester.widgetList<ScaleTransition>(scaleFinder).first;
    final scale2 = scaleTransition2.scale.value;

    // Scale should have changed
    expect(scale1, isNot(scale2));
  });
}
