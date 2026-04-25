import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/services/tooltip_registry.dart';

void main() {
  group('TooltipRegistry', () {
    test('get returns registered tooltip for known key', () {
      expect(
        TooltipRegistry.get('global_help'),
        'Open the Help Center to find answers and guides for your business.',
      );
    });

    test('get returns fallback for unknown key', () {
      expect(
        TooltipRegistry.get('unknown_key'),
        'More information',
      );
    });

    test('get returns custom fallback when provided', () {
      expect(
        TooltipRegistry.get('unknown_key', fallback: 'Custom fallback'),
        'Custom fallback',
      );
    });
  });

  group('RegisteredTooltip', () {
    testWidgets('renders child and tooltip correctly', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: RegisteredTooltip(
              tooltipKey: 'global_help',
              child: const Text('Test Child'),
            ),
          ),
        ),
      );

      // Verify the child is rendered
      expect(find.text('Test Child'), findsOneWidget);

      // Verify a Tooltip widget exists wrapping the child
      final Tooltip tooltip = tester.widget(find.byType(Tooltip));
      expect(
        tooltip.message,
        'Open the Help Center to find answers and guides for your business.',
      );
      expect(tooltip.triggerMode, TooltipTriggerMode.longPress);
    });
  });
}
