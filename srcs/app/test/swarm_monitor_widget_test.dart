import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/swarm_monitor_widget.dart';

void main() {
  testWidgets('SwarmMonitorWidget renders correctly and shows connecting state initially', (WidgetTester tester) async {
    // Provide an invalid URL so it doesn't try to connect to a real server during testing,
    // or just let it fail gracefully. We are primarily testing the UI rendering here.
    await tester.pumpWidget(const MaterialApp(home: Scaffold(body: SwarmMonitorWidget(wsUrl: 'ws://invalid.local'))));

    expect(find.text('Swarm Agent Status'), findsOneWidget);
    expect(find.byType(GlassCard), findsOneWidget);
    // Initially it should show connecting
    expect(find.text('Connecting...'), findsOneWidget);

    // To resolve the active timer issue for testing we can dispose it or pump the duration
    await tester.pump(const Duration(seconds: 3));
    await tester.pump(const Duration(seconds: 3));
  });
}
