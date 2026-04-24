import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/hybrid_agent_status.dart';

void main() {
  testWidgets('HybridAgentStatusWidget renders Cloud Synced when true', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: HybridAgentStatusWidget(isCloudSynced: true),
        ),
      ),
    );

    await tester.pump(); // Start animation

    expect(find.text('Agent Status'), findsOneWidget);
    expect(find.text('Cloud Synced'), findsOneWidget);
    
    final textFinder = find.text('Cloud Synced');
    final textWidget = tester.widget<Text>(textFinder);
    expect(textWidget.style!.color, Colors.greenAccent);
  });

  testWidgets('HybridAgentStatusWidget renders Standalone when false', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: HybridAgentStatusWidget(isCloudSynced: false),
        ),
      ),
    );

    await tester.pump();

    expect(find.text('Agent Status'), findsOneWidget);
    expect(find.text('Standalone'), findsOneWidget);
    
    final textFinder = find.text('Standalone');
    final textWidget = tester.widget<Text>(textFinder);
    expect(textWidget.style!.color, Colors.yellowAccent);
  });

  testWidgets('HybridAgentStatusWidget pulses', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: HybridAgentStatusWidget(isCloudSynced: true),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final containerFinder = find.byType(Container);
    // There should be at least 2 containers. The second one is the pulsing circle.
    expect(containerFinder, findsWidgets);
    
    final container1 = tester.widget<Container>(containerFinder.at(1));
    final decoration1 = container1.decoration as BoxDecoration;
    final opacity1 = decoration1.color!.opacity;

    // Advance time by 1s (half of duration)
    await tester.pump(const Duration(seconds: 1));

    final container2 = tester.widget<Container>(containerFinder.at(1));
    final decoration2 = container2.decoration as BoxDecoration;
    final opacity2 = decoration2.color!.opacity;

    // Opacity should have changed
    expect(opacity1, isNot(opacity2));
  });
}
