import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

void main() {
  testWidgets('HelpCenterScreen renders and searches correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(
      home: HelpCenterScreen(),
    ));

    // Verify initial load
    expect(find.text('Help Center'), findsOneWidget);
    expect(find.text('Getting Started with One Human Corp'), findsOneWidget);

    // Test search
    await tester.enterText(find.byType(TextField), 'stripe');
    await tester.pumpAndSettle();

    expect(find.text('Accepting Payments with Stripe'), findsOneWidget);
    expect(find.text('Getting Started with One Human Corp'), findsNothing);
  });

  testWidgets('HelpTooltip displays correct message', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        body: Center(
          child: HelpTooltip(
            tooltipKey: 'dashboard_help',
            child: const Icon(Icons.help),
          ),
        ),
      ),
    ));

    final tooltipFinder = find.byType(Tooltip);
    expect(tooltipFinder, findsOneWidget);

    final Tooltip tooltipWidget = tester.widget(tooltipFinder);
    expect(tooltipWidget.message, 'Open the Help Center for guides and support.');
  });
}
