import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders and applies hover scale', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            child: const Text('Test Card Content'),
          ),
        ),
      ),
    );

    expect(find.text('Test Card Content'), findsOneWidget);

    // Initial scale is 1.0
    final animatedScaleFinder = find.byType(AnimatedScale);
    expect(animatedScaleFinder, findsOneWidget);

    AnimatedScale animatedScale = tester.widget(animatedScaleFinder);
    expect(animatedScale.scale, 1.0);

    // Hover over the card
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    addTearDown(gesture.removePointer);
    await tester.pump();

    // Move to the center of the GlassCard
    await gesture.moveTo(tester.getCenter(find.byType(GlassCard)));
    await tester.pumpAndSettle();

    animatedScale = tester.widget(animatedScaleFinder);
    expect(animatedScale.scale, 1.02);

    // Move away completely
    await gesture.moveTo(const Offset(1000, 1000));
    await tester.pumpAndSettle();

    animatedScale = tester.widget(animatedScaleFinder);
    expect(animatedScale.scale, 1.0);
  });
}
