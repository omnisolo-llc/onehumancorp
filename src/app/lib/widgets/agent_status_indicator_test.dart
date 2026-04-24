import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/agent_status_indicator.dart';

void main() {
  testWidgets('AgentStatusIndicator renders green when active', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentStatusIndicator(isActive: true),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final finder = find.byWidgetPredicate((widget) {
      if (widget is Container) {
        final dec = widget.decoration;
        if (dec is BoxDecoration) {
          return dec.color == Colors.greenAccent;
        }
      }
      return false;
    });

    expect(finder, findsOneWidget);
  });

  testWidgets('AgentStatusIndicator renders grey when inactive', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentStatusIndicator(isActive: false),
        ),
      ),
    );

    await tester.pump();

    final finder = find.byWidgetPredicate((widget) {
      if (widget is Container) {
        final dec = widget.decoration;
        if (dec is BoxDecoration) {
          return dec.color == Colors.grey;
        }
      }
      return false;
    });

    expect(finder, findsOneWidget);
  });

  testWidgets('AgentStatusIndicator pulses when active', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentStatusIndicator(isActive: true),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final finder = find.byWidgetPredicate((widget) {
      if (widget is Container) {
        final dec = widget.decoration;
        if (dec is BoxDecoration) {
          return dec.color == Colors.greenAccent;
        }
      }
      return false;
    });

    final container1 = tester.widget<Container>(finder);
    final decoration1 = container1.decoration as BoxDecoration;
    final boxShad1 = decoration1.boxShadow!.first;

    // Advance time by 750ms (half of duration)
    await tester.pump(const Duration(milliseconds: 750));

    final container2 = tester.widget<Container>(finder);
    final decoration2 = container2.decoration as BoxDecoration;
    final boxShad2 = decoration2.boxShadow!.first;

    // Blur radius should have changed due to pulsing
    expect(boxShad1.blurRadius, isNot(boxShad2.blurRadius));
  });

  testWidgets('AgentStatusIndicator stops pulsing when becoming inactive', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentStatusIndicator(isActive: true),
        ),
      ),
    );

    await tester.pump(); // Start animation

    // Update widget to inactive
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentStatusIndicator(isActive: false),
        ),
      ),
    );

    await tester.pump();

    final finder = find.byWidgetPredicate((widget) {
      if (widget is Container) {
        final dec = widget.decoration;
        if (dec is BoxDecoration) {
          return dec.color == Colors.grey;
        }
      }
      return false;
    });

    final container = tester.widget<Container>(finder);
    final decoration = container.decoration as BoxDecoration;

    // Should have no box shadow when inactive
    expect(decoration.boxShadow, isEmpty);
  });
}
