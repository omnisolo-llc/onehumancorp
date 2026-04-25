import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/help/tooltip_registry.dart';
import 'package:ohc_app/widgets/help/ohc_tooltip.dart';
import 'package:ohc_app/widgets/help/walkthrough_overlay.dart';

void main() {
  group('OhcTooltip Widget Tests', () {
    testWidgets('renders child widget', (WidgetTester tester) async {
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: OhcTooltip(
              tooltipKey: 'test_key_not_in_registry',
              child: Text('Tooltip Child'),
            ),
          ),
        ),
      );

      expect(find.text('Tooltip Child'), findsOneWidget);
    });
  });

  group('WalkthroughOverlay Widget Tests', () {
    testWidgets('renders overlay without crashing', (WidgetTester tester) async {
      bool completed = false;

      // Use a completely unattached GlobalKey so it safely falls back to screen center
      // without triggering any "deactivated ancestor" issues during test tree build.
      final steps = [
        WalkthroughStep(key: GlobalKey(), title: 'Step 1', description: 'Desc 1'),
      ];

      // Provide a real Material app with proper constraints and layout so Positioned.fill works
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Stack(
              children: [
                WalkthroughOverlay(
                  steps: steps,
                  onComplete: () => completed = true,
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Verify the Step 1 text renders in the WalkthroughOverlay
      expect(find.text('Step 1'), findsOneWidget);
      expect(find.text('Desc 1'), findsOneWidget);
      expect(find.text('Finish'), findsOneWidget);

      await tester.tap(find.text('Finish'));
      expect(completed, isTrue);
    });
  });
}
