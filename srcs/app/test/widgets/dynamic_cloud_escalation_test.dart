import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/dynamic_cloud_escalation.dart';

void main() {
  testWidgets('DynamicCloudEscalationWidget renders correctly in local state', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: DynamicCloudEscalationWidget(state: EscalationState.local),
        ),
      ),
    );

    expect(find.text('Hybrid MCP RAG'), findsOneWidget);
    expect(find.text('Local SQLite (Private)'), findsOneWidget);

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump();
  });

  testWidgets('DynamicCloudEscalationWidget renders correctly in escalating state', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: DynamicCloudEscalationWidget(state: EscalationState.escalating),
        ),
      ),
    );

    expect(find.text('Hybrid MCP RAG'), findsOneWidget);
    expect(find.text('Escalating Workload...'), findsOneWidget);

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump();
  });

  testWidgets('DynamicCloudEscalationWidget renders correctly in cloud state', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: DynamicCloudEscalationWidget(state: EscalationState.cloud),
        ),
      ),
    );

    expect(find.text('Hybrid MCP RAG'), findsOneWidget);
    expect(find.text('Cloud Swarm (Infinite Scale)'), findsOneWidget);

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump();
  });
}
