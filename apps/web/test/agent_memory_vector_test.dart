import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/agent_memory_vector.dart';

void main() {
  testWidgets('AgentMemoryVectorWidget renders vector data correctly', (WidgetTester tester) async {
    final testVector = [0.1, -0.5, 0.8, -0.2, 0.0];

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: AgentMemoryVectorWidget(vectorData: testVector),
        ),
      ),
    );

    expect(find.text('AutoDream Memory Vector'), findsOneWidget);

    // There are 5 items in the vector
    expect(find.byType(Tooltip), findsNWidgets(5));
  });
}
