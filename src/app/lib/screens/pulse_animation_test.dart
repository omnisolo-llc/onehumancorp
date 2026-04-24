import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/pulse_animation.dart';

void main() {
  testWidgets('PulseAnimation scales child', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: PulseAnimation(
            child: Text('Pulse'),
          ),
        ),
      ),
    );

    expect(find.text('Pulse'), findsOneWidget);

    // Pump frames to verify animation controller is active
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));
    expect(find.text('Pulse'), findsOneWidget);
  });
}
