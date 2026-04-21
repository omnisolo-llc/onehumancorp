import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/autodream_sync_walkthrough_screen.dart';

void main() {
  group('AutoDreamSyncWalkthroughScreen Widget Tests', () {
    testWidgets('renders basic UI structure and navigates through steps', (tester) async {
      tester.view.physicalSize = const Size(2400, 1600);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);

      await tester.pumpWidget(
        const MaterialApp(
          home: AutoDreamSyncWalkthroughScreen(),
        ),
      );

      // Verify Screen title
      expect(find.text('AutoDream Sync Daemon Walkthrough'), findsOneWidget);
      expect(find.text('Interactive Guide: Sync Lifecycle'), findsOneWidget);

      // Verify Step 1
      expect(find.text('1. Generate & Insert Vector'), findsOneWidget);
      expect(find.text('Step 1 of 7'), findsOneWidget);

      // Tap Next Step
      final nextButton = find.text('Next Step');
      expect(nextButton, findsOneWidget);

      await tester.tap(nextButton);
      await tester.pumpAndSettle();

      // Verify Step 2
      expect(find.text('2. Query Pending Vectors'), findsOneWidget);
      expect(find.text('Step 2 of 7'), findsOneWidget);

      // Tap Previous Step
      final prevButton = find.text('Previous Step');
      expect(prevButton, findsOneWidget);

      await tester.tap(prevButton);
      await tester.pumpAndSettle();

      // Verify back to Step 1
      expect(find.text('1. Generate & Insert Vector'), findsOneWidget);
      expect(find.text('Step 1 of 7'), findsOneWidget);

      // Navigate to the end
      for (int i = 0; i < 6; i++) {
        await tester.tap(nextButton);
        await tester.pumpAndSettle();
      }

      // Verify Step 7
      expect(find.text('7. Update sync_status'), findsOneWidget);
      expect(find.text('Step 7 of 7'), findsOneWidget);
    });
  });
}
