import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:billing/widgets/token_budget_visualizer.dart';

void main() {
  testWidgets('TokenBudgetVisualizer renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: Scaffold(
        body: TokenBudgetVisualizer(tokenBudget: 500000),
      ),
    ));

    expect(find.text('Token Budget Analytics'), findsOneWidget);
    expect(find.text('Available Budget: 500000'), findsOneWidget);
  });
}
