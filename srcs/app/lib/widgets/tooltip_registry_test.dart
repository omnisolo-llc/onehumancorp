import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

void main() {
  testWidgets('TooltipRegistry renders child and shows message on long press', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: TooltipRegistry(
            message: 'Test Tooltip',
            child: Icon(Icons.star),
          ),
        ),
      ),
    );

    // Verify child is rendered
    expect(find.byIcon(Icons.star), findsOneWidget);

    // Long press to trigger tooltip
    await tester.longPress(find.byIcon(Icons.star));
    await tester.pumpAndSettle();

    // Verify tooltip text is displayed
    expect(find.text('Test Tooltip'), findsOneWidget);
  });
}
