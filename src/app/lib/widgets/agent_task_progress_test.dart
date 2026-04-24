import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/agent_task_progress.dart';

void main() {
  testWidgets('AgentTaskProgressWidget renders task name and progress', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Coding Task',
            progress: 0.5,
            isWorking: false,
          ),
        ),
      ),
    );

    await tester.pump(); // Start animation if any

    expect(find.text('Coding Task'), findsOneWidget);

    final progressFinder = find.byType(LinearProgressIndicator);
    expect(progressFinder, findsOneWidget);

    final progressIndicator = tester.widget<LinearProgressIndicator>(progressFinder);
    expect(progressIndicator.value, 0.5);
  });

  testWidgets('AgentTaskProgressWidget glows when working', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Coding Task',
            progress: 0.5,
            isWorking: true,
          ),
        ),
      ),
    );

    await tester.pump(); // Start animation

    final finder = find.byWidgetPredicate((widget) {
      if (widget is Container) {
        final dec = widget.decoration;
        if (dec is BoxDecoration) {
          final border = dec.border;
          if (border is Border) {
            return border.top.color == const Color.fromRGBO(255, 255, 255, 0.1);
          }
        }
      }
      return false;
    });

    final container1 = tester.widget<Container>(finder);
    final decoration1 = container1.decoration as BoxDecoration;
    final color1 = decoration1.color!;

    // Advance time by 500ms (half of duration)
    await tester.pump(const Duration(milliseconds: 500));

    final container2 = tester.widget<Container>(finder);
    final decoration2 = container2.decoration as BoxDecoration;
    final color2 = decoration2.color!;

    // Opacity should have changed due to glowing
    expect(color1.opacity, isNot(color2.opacity));
  });

  testWidgets('AgentTaskProgressWidget stops glowing when not working', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Coding Task',
            progress: 0.5,
            isWorking: true,
          ),
        ),
      ),
    );

    await tester.pump(); // Start animation

    // Update widget to not working
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: AgentTaskProgressWidget(
            taskName: 'Coding Task',
            progress: 0.5,
            isWorking: false,
          ),
        ),
      ),
    );

    await tester.pump();

    final finder = find.byWidgetPredicate((widget) {
      if (widget is Container) {
        final dec = widget.decoration;
        if (dec is BoxDecoration) {
          final border = dec.border;
          if (border is Border) {
            return border.top.color == const Color.fromRGBO(255, 255, 255, 0.1);
          }
        }
      }
      return false;
    });

    final container = tester.widget<Container>(finder);
    final decoration = container.decoration as BoxDecoration;

    // Opacity should be at minimum (0.03) when stopped
    expect(decoration.color!.opacity, closeTo(0.03, 0.005));
  });
}
