import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/agent_mesh_message_tile.dart';

void main() {
  testWidgets('AgentMeshMessageTile renders correctly', (WidgetTester tester) async {
    final timestamp = DateTime(2023, 10, 10, 14, 30);

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AgentMeshMessageTile(
            sender: 'Agent Alpha',
            message: 'Data processing complete',
            timestamp: timestamp,
          ),
        ),
      ),
    );

    expect(find.text('Agent Alpha'), findsOneWidget);
    expect(find.text('Data processing complete'), findsOneWidget);
    expect(find.text('14:30'), findsOneWidget);

    final container = tester.widget<Container>(find.byType(Container).first);
    final decoration = container.decoration as BoxDecoration;
    expect(decoration.color, const Color.fromRGBO(255, 255, 255, 0.03));

    await tester.pumpAndSettle();
  });
}
