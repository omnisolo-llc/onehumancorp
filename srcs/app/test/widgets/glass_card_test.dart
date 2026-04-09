import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders and responds to hover', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Center(
            child: GlassCard(
              child: const Text('Hello Glass'),
            ),
          ),
        ),
      ),
    );

    // Verify text is rendered
    expect(find.text('Hello Glass'), findsOneWidget);

    // Verify scale is 1.0 initially
    final initialScale = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(initialScale.scale, 1.0);

    // Simulate mouse hover
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    addTearDown(gesture.removePointer);
    await tester.pump();
    await gesture.moveTo(tester.getCenter(find.byType(GlassCard)));
    await tester.pumpAndSettle();

    // Verify scale is 1.02 on hover
    final hoverScale = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(hoverScale.scale, 1.02);
  });
}
