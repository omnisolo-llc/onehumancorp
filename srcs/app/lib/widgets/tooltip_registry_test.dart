import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

void main() {
  test('TooltipRegistry.get returns correct text for existing key', () {
    final text = TooltipRegistry.get('dashboard_refresh');
    expect(text, 'Update the information shown on your dashboard.');
  });

  test('TooltipRegistry.get returns fallback for non-existing key', () {
    final text = TooltipRegistry.get('non_existing_key');
    expect(text, 'Tooltip: non_existing_key');
  });

  testWidgets('RegisteredTooltip renders child and shows message', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: RegisteredTooltip(
            tooltipKey: 'dashboard_refresh',
            child: Text('Target'),
          ),
        ),
      ),
    );

    expect(find.text('Target'), findsOneWidget);

    final tooltipFinder = find.byType(Tooltip);
    expect(tooltipFinder, findsOneWidget);

    final tooltip = tester.widget<Tooltip>(tooltipFinder);
    expect(tooltip.message, 'Update the information shown on your dashboard.');
  });
}
