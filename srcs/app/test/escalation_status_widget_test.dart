import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/escalation_status_widget.dart';

void main() {
  testWidgets('EscalationStatusWidget displays local status', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: EscalationStatusWidget(isEscalated: false),
        ),
      ),
    );

    expect(find.text('Task Status'), findsOneWidget);
    expect(find.text('Local SQLite'), findsOneWidget);
    expect(find.byIcon(Icons.computer), findsOneWidget);
  });

  testWidgets('EscalationStatusWidget displays cloud status', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: EscalationStatusWidget(isEscalated: true),
        ),
      ),
    );

    expect(find.text('Task Status'), findsOneWidget);
    expect(find.text('Cloud Orchestration'), findsOneWidget);
    expect(find.byIcon(Icons.cloud), findsOneWidget);
  });
}
