import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:billing/widgets/billing_dashboard.dart';

void main() {
  testWidgets('BillingDashboard renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: BillingDashboard(),
      ),
    ));

    expect(find.text('Billing Dashboard'), findsOneWidget);
    expect(find.text('Current Plan: PRO'), findsOneWidget);
  });
}
