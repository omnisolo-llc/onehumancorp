import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/autodream_pipeline_widget.dart';

void main() {
  testWidgets('AutoDreamPipelineWidget renders title and nodes', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AutoDreamPipelineWidget(),
        ),
      ),
    );

    await tester.pump(); // Start animation

    expect(find.text('AutoDream Pipeline Stream'), findsOneWidget);
    expect(find.text('Extract'), findsOneWidget);
    expect(find.text('Analyze'), findsOneWidget);
    expect(find.text('Embed'), findsOneWidget);
    expect(find.text('Store'), findsOneWidget);
  });

  testWidgets('AutoDreamPipelineWidget renders connections with CustomPaint', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AutoDreamPipelineWidget(),
        ),
      ),
    );

    await tester.pump();

    final customPaintFinder = find.byType(CustomPaint);
    // We expect at least 3 connections
    expect(customPaintFinder, findsWidgets);
    final count = tester.widgetList(customPaintFinder).length;
    expect(count, greaterThanOrEqualTo(3));
  });

  testWidgets('AutoDreamPipelineWidget animates connections', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AutoDreamPipelineWidget(),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final customPaintFinder = find.byType(CustomPaint);
    // Find the ones that use _ConnectionPainter
    // Since we can't easily cast to private type, we just check that progress changes if we can access it via dynamic.
    // Or we can just check that the widget repaints.

    final customPaints = tester.widgetList<CustomPaint>(customPaintFinder).toList();
    // Let's try to find the one that has a painter with 'progress' property.
    dynamic targetedPainter;
    for (final cp in customPaints) {
      final painter = cp.painter;
      try {
        if ((painter as dynamic).progress != null) {
          targetedPainter = painter;
          break;
        }
      } catch (_) {
        // Ignore if property doesn't exist
      }
    }

    expect(targetedPainter, isNotNull, reason: 'Could not find _ConnectionPainter');

    final progress1 = targetedPainter.progress;

    // Advance time by 1.5s (half of duration)
    await tester.pump(const Duration(milliseconds: 1500));

    // Re-query to get the new painter instance
    final customPaintsAfter = tester.widgetList<CustomPaint>(customPaintFinder).toList();
    dynamic targetedPainterAfter;
    for (final cp in customPaintsAfter) {
      final painter = cp.painter;
      try {
        if ((painter as dynamic).progress != null) {
          targetedPainterAfter = painter;
          break;
        }
      } catch (_) {}
    }

    expect(targetedPainterAfter, isNotNull, reason: 'Could not find _ConnectionPainter after pump');
    final progress2 = targetedPainterAfter.progress;

    // Progress should have changed
    expect(progress1, isNot(progress2));
  });
}
