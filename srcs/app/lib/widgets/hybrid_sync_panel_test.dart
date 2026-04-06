import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/hybrid_sync_panel.dart';

void main() {
  group('HybridSyncPanelWidget Tests', () {
    testWidgets('renders basic UI structure and texts', (tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: HybridSyncPanelWidget(),
          ),
        ),
      );

      // Verify Screen title
      expect(find.text('Omni-Context Sync'), findsOneWidget);
      expect(find.text('Active'), findsOneWidget);
      expect(
        find.text(
            'Bridging Standalone Local SQLite with Multi-Tenant Cloud Postgres. Private RAG execution with Cloud Escalation.'),
        findsOneWidget,
      );

      // Verify stats
      expect(find.text('Local Vectors'), findsOneWidget);
      expect(find.text('1,402'), findsOneWidget);
      expect(find.text('Cloud Escalations'), findsOneWidget);
      expect(find.text('34'), findsOneWidget);
      expect(find.text('Last Sync'), findsOneWidget);
      expect(find.text('Just now'), findsOneWidget);

      // Verify icons
      expect(find.byIcon(Icons.cloud_sync), findsOneWidget);
      expect(find.byIcon(Icons.check_circle), findsOneWidget);
      expect(find.byIcon(Icons.data_usage), findsOneWidget);
      expect(find.byIcon(Icons.arrow_upward), findsOneWidget);
      expect(find.byIcon(Icons.access_time), findsOneWidget);
    });
  });
}
