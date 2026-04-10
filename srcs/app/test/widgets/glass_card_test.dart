import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders child and handles margin', (WidgetTester tester) async {
    const childKey = Key('child');
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassCard(
            margin: EdgeInsets.all(16.0),
            child: Text('Test', key: childKey),
          ),
        ),
      ),
    );

    expect(find.byKey(childKey), findsOneWidget);
    expect(find.text('Test'), findsOneWidget);

    final paddingFinder = find.ancestor(of: find.byType(MouseRegion), matching: find.byType(Padding));
    final paddingWidget = tester.widget<Padding>(paddingFinder.first);
    expect(paddingWidget.padding, const EdgeInsets.all(16.0));
  });

  testWidgets('GlassCard responds to hover', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: Center(
            child: GlassCard(
              child: SizedBox(width: 100, height: 100),
            ),
          ),
        ),
      ),
    );

    final scaleFinder = find.byType(AnimatedScale);
    var scaleWidget = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget.scale, 1.0);

    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: Offset.zero);
    addTearDown(gesture.removePointer);
    await tester.pump();

    await gesture.moveTo(tester.getCenter(find.byType(GlassCard)));
    await tester.pumpAndSettle();

    scaleWidget = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget.scale, 1.02);

    await gesture.moveTo(const Offset(0, 0));
    await tester.pumpAndSettle();

    scaleWidget = tester.widget<AnimatedScale>(scaleFinder);
    expect(scaleWidget.scale, 1.0);
  });
}
