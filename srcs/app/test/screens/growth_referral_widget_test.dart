import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/user_management_screen.dart';

void main() {
  testWidgets('GrowthReferralWidget renders correctly and responds to hover', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        ),
        home: const Scaffold(
          body: Center(
            child: GrowthReferralWidget(),
          ),
        ),
      ),
    );

    // Initial render check
    expect(find.text('Grow Your Swarm. Maintain Sovereignty.'), findsOneWidget);
    expect(find.byIcon(Icons.group_add), findsOneWidget);

    // Initial state: AnimatedScale should have scale 1.0
    final AnimatedScale scaleWidget = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(scaleWidget.scale, 1.0);

    // Simulate hover enter
    final gesture = await tester.createGesture(kind: PointerDeviceKind.mouse);
    await gesture.addPointer(location: tester.getCenter(find.byType(GrowthReferralWidget)));
    await tester.pumpAndSettle();

    // Check if hovered state is applied
    final AnimatedScale hoverScaleWidget = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(hoverScaleWidget.scale, 1.02);

    // Simulate hover exit - remove pointer
    await gesture.removePointer();
    await tester.pumpAndSettle();

    final AnimatedScale exitScaleWidget = tester.widget<AnimatedScale>(find.byType(AnimatedScale));
    expect(exitScaleWidget.scale, 1.0);
  });
}
