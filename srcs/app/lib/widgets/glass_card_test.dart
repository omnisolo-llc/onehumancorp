import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders child and applies padding', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassCard(
            padding: EdgeInsets.all(10),
            child: Text('Hello'),
          ),
        ),
      ),
    );

    expect(find.text('Hello'), findsOneWidget);

    final containerFinder = find.byType(AnimatedContainer);
    expect(containerFinder, findsOneWidget);

    final container = tester.widget<AnimatedContainer>(containerFinder);
    expect(container.padding, const EdgeInsets.all(10));
  });

  testWidgets('GlassCard applies margin', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassCard(
            margin: EdgeInsets.all(15),
            child: Text('Hello'),
          ),
        ),
      ),
    );

    final paddingFinder = find.byType(Padding).first; // Outer padding for margin
    final padding = tester.widget<Padding>(paddingFinder);
    expect(padding.padding, const EdgeInsets.all(15));
  });

  testWidgets('GlassCard scales on hover', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassCard(
            child: Text('Hello'),
          ),
        ),
      ),
    );

    final scaleFinder = find.byType(AnimatedScale);
    expect(scaleFinder, findsOneWidget);

    final scaleWidget1 = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget1.scale, 1.0);

    // Simulate mouse enter
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    await gesture.moveTo(tester.getCenter(find.text('Hello')));
    await tester.pumpAndSettle();

    final scaleWidget2 = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget2.scale, 1.02);

    // Simulate mouse exit
    await gesture.moveTo(const Offset(1000, 1000));
    await tester.pumpAndSettle();

    final scaleWidget3 = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget3.scale, 1.0);
  });

  testWidgets('GlassCard changes background color on hover', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassCard(
            child: Text('Hello'),
          ),
        ),
      ),
    );

    final containerFinder = find.byType(AnimatedContainer);
    final container1 = tester.widget<AnimatedContainer>(containerFinder);
    final decoration1 = container1.decoration as BoxDecoration;
    expect(decoration1.color, const Color.fromRGBO(255, 255, 255, 0.03));

    // Simulate mouse enter
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    await gesture.moveTo(tester.getCenter(find.text('Hello')));
    await tester.pumpAndSettle();

    final container2 = tester.widget<AnimatedContainer>(containerFinder);
    final decoration2 = container2.decoration as BoxDecoration;
    expect(decoration2.color, const Color.fromRGBO(255, 255, 255, 0.08));
  });
}
