import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/registry_tooltip.dart';
import 'package:ohc_app/services/tooltip_registry.dart';

void main() {
  testWidgets('RegistryTooltip displays correct message', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: RegistryTooltip(
            tooltipKey: 'dashboard_agents',
            child: Text('Hover Me'),
          ),
        ),
      ),
    );

    expect(find.text('Hover Me'), findsOneWidget);
    // Tooltip message check can be tricky, but we verify the widget is created.
    final tooltip = tester.widget<Tooltip>(find.byType(Tooltip));
    expect(tooltip.message, TooltipRegistry.get('dashboard_agents'));
  });

  test('TooltipRegistry returns fallback for unknown key', () {
    expect(TooltipRegistry.get('unknown_key'), 'Help information unavailable.');
  });
}
