import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders and responds to hover/tap', (WidgetTester tester) async {
    bool tapped = false;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            margin: const EdgeInsets.all(8),
            onTap: () {
              tapped = true;
            },
            child: const Text('Hello Glass'),
          ),
        ),
      ),
    );

    expect(find.text('Hello Glass'), findsOneWidget);
    expect(find.byType(GlassCard), findsOneWidget);

    await tester.tap(find.byType(GlassCard));
    await tester.pumpAndSettle();

    expect(tapped, isTrue);

    // Hover test
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    addTearDown(gesture.removePointer);

    await tester.pump();
    await gesture.moveTo(tester.getCenter(find.byType(GlassCard)));
    await tester.pumpAndSettle();

    // Check if AnimatedScale scales to 1.02
    final scaleWidget = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(scaleWidget.scale, 1.02);

    await gesture.moveTo(Offset.zero);
    await tester.pumpAndSettle();

    final scaleWidgetOut = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(scaleWidgetOut.scale, 1.0);
  });
}
