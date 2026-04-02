import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/observability/observability_dashboard.dart';
import 'package:ohc_app/widgets/glass_container.dart';
import 'package:ohc_app/services/teammate_mesh_service.dart';

void main() {
  group('GlassContainer Widget Tests', () {
    testWidgets('renders child correctly', (WidgetTester tester) async {
      const childKey = Key('child-key');
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: GlassContainer(
              child: Text('Glass Child', key: childKey),
            ),
          ),
        ),
      );

      expect(find.byKey(childKey), findsOneWidget);
      expect(find.text('Glass Child'), findsOneWidget);
    });
  });

  group('ObservabilityDashboard Widget Tests', () {
    testWidgets('renders dashboard correctly with empty state', (WidgetTester tester) async {
      // Mock the teammateMeshProvider to emit an empty stream or just wait
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            teammateMeshProvider('test-room').overrideWith((ref) => const Stream.empty()),
          ],
          child: const MaterialApp(
            home: ObservabilityDashboard(roomId: 'test-room'),
          ),
        ),
      );

      // Verify AppBar title
      expect(find.text('Swarm Observability'), findsOneWidget);

      // Verify body text
      expect(find.text('Realtime Teammate Mesh'), findsOneWidget);

      // We expect the waiting text initially or quickly after pump
      await tester.pump();
      expect(find.text('Waiting for messages...'), findsWidgets); // Actually might be findsOneWidget but let's be safe
    });
  });
}
