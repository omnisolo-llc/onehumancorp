import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders child and handles padding', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            padding: const EdgeInsets.all(10.0),
            child: const Text('GlassCard Child'),
          ),
        ),
      ),
    );

    expect(find.text('GlassCard Child'), findsOneWidget);

    // Test hover scale using mouse interaction
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    addTearDown(gesture.removePointer);

    // Verify default scale is 1.0 (though it's animated, it starts at 1.0)
    final animatedScale = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(animatedScale.scale, 1.0);

    // Hover over the card
    await gesture.moveTo(tester.getCenter(find.byType(GlassCard)));
    await tester.pumpAndSettle();

    // Scale should now be 1.02
    final animatedScaleHovered = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(animatedScaleHovered.scale, 1.02);
  });
}
