import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../dashboard_widget.dart';

void main() {
  testWidgets('WebDashboardWidget renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: WebDashboardWidget(
            title: 'Test Title',
            value: '42',
            icon: Icons.speed,
          ),
        ),
      ),
    );

    expect(find.text('Test Title'), findsOneWidget);
    expect(find.text('42'), findsOneWidget);
    expect(find.byIcon(Icons.speed), findsOneWidget);
  });
}
