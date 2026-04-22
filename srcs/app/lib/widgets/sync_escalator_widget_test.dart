import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'sync_escalator_widget.dart';

void main() {
  group('SyncEscalatorWidget', () {
    testWidgets('renders local state correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: SyncEscalatorWidget(isCloudEscalated: false),
          ),
        ),
      );

      expect(find.text('MCP RAG Execution'), findsOneWidget);
      expect(find.text('LOCAL ONLY'), findsOneWidget);
      expect(find.text('Running privately and locally via SQLite.'), findsOneWidget);

      final iconFinder = find.byIcon(Icons.lock_outline);
      expect(iconFinder, findsOneWidget);
    });

    testWidgets('renders cloud escalated state correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: SyncEscalatorWidget(isCloudEscalated: true),
          ),
        ),
      );

      expect(find.text('MCP RAG Execution'), findsOneWidget);
      expect(find.text('CLOUD SWARM'), findsOneWidget);
      expect(find.text('Workload escalated to cloud swarm for massively parallel computation.'), findsOneWidget);

      final iconFinder = find.byIcon(Icons.cloud_queue);
      expect(iconFinder, findsOneWidget);
    });
  });
}
