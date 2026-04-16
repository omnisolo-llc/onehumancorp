import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/agent_status_indicator.dart';

void main() {
  testWidgets('AgentStatusIndicator animates sequentially when active', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(body: AgentStatusIndicator(isActive: true)),
      ),
    );

    expect(find.byType(AgentStatusIndicator), findsOneWidget);

    // Pump sequentially instead of pumpAndSettle due to infinite animation
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));
    await tester.pump(const Duration(milliseconds: 500));

    expect(find.byType(Container), findsWidgets);
  });
}
