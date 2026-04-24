import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/hybrid_agent_status.dart';

void main() {
  testWidgets('HybridAgentStatusWidget renders correctly when synced', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(body: HybridAgentStatusWidget(isCloudSynced: true)),
      ),
    );

    expect(find.text('Agent Status'), findsOneWidget);
    expect(find.text('Cloud Synced'), findsOneWidget);

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump();
  });

  testWidgets('HybridAgentStatusWidget renders correctly when standalone', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(body: HybridAgentStatusWidget(isCloudSynced: false)),
      ),
    );

    expect(find.text('Agent Status'), findsOneWidget);
    expect(find.text('Standalone'), findsOneWidget);

    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump();
  });
}
