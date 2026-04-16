import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/sync_escalator_status_widget.dart';

void main() {
  testWidgets('SyncEscalatorStatusWidget shows local mode initially', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SyncEscalatorStatusWidget(
            isCloudEscalated: false,
            escalatedTaskCount: 0,
          ),
        ),
      ),
    );

    expect(find.text('Hybrid RAG Escalation Status'), findsOneWidget);
    expect(find.text('Local Default (SQLite)'), findsOneWidget);
    expect(find.text('Escalated Tasks: 0'), findsOneWidget);
    expect(find.byIcon(Icons.lock_outline), findsOneWidget);
  });

  testWidgets('SyncEscalatorStatusWidget shows cloud mode when escalated', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SyncEscalatorStatusWidget(
            isCloudEscalated: true,
            escalatedTaskCount: 5,
          ),
        ),
      ),
    );

    expect(find.text('Cloud Swarm (PostgreSQL)'), findsOneWidget);
    expect(find.text('Escalated Tasks: 5'), findsOneWidget);
    expect(find.byIcon(Icons.cloud_sync_outlined), findsOneWidget);
  });
}
