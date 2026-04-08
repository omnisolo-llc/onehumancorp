import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  group('GlassCard Widget', () {
    testWidgets('renders child content correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: GlassCard(
              child: const Text('Glass Content'),
            ),
          ),
        ),
      );

      expect(find.text('Glass Content'), findsOneWidget);
    });

    testWidgets('responds to hover by changing scale', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Center(
              child: GlassCard(
                child: const SizedBox(width: 100, height: 100),
              ),
            ),
          ),
        ),
      );

      // Find the AnimatedScale widget which is a child of GlassCard
      Finder scaleFinder = find.byType(AnimatedScale);
      expect(scaleFinder, findsOneWidget);
      AnimatedScale animatedScale = tester.widget<AnimatedScale>(scaleFinder);
      expect(animatedScale.scale, 1.0); // Initial scale

      // Simulate hover
      final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
      await gesture.addPointer(location: tester.getCenter(find.byType(GlassCard)));
      await tester.pumpAndSettle();

      animatedScale = tester.widget<AnimatedScale>(scaleFinder);
      expect(animatedScale.scale, 1.02); // Scaled up

      // Simulate exit hover
      await gesture.removePointer();
      await tester.pumpAndSettle();

      animatedScale = tester.widget<AnimatedScale>(scaleFinder);
      expect(animatedScale.scale, 1.0); // Scaled down
    });
  });
}
