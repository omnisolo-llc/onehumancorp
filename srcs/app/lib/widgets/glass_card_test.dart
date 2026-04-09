import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders and applies scale on hover', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassCard(
            child: Text('Glass Card Content'),
          ),
        ),
      ),
    );

    // Verify initial state
    expect(find.text('Glass Card Content'), findsOneWidget);

    // Find the AnimatedScale widget to check its scale
    final scaleFinder = find.byType(AnimatedScale);
    expect(scaleFinder, findsOneWidget);

    AnimatedScale scaleWidget = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget.scale, 1.0);

    // Hover over the GlassCard
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    await tester.pump();

    // Move pointer over the widget
    await gesture.moveTo(tester.getCenter(find.text('Glass Card Content')));
    await tester.pumpAndSettle();

    // Verify hover scale
    scaleWidget = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget.scale, 1.02);
  });
}
