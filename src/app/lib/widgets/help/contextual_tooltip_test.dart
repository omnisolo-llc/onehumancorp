import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/help/contextual_tooltip.dart';

void main() {
  testWidgets('ContextualTooltip renders child and has correct tooltip message', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: ContextualTooltip(
            id: 'dashboard_title',
            child: Text('Test Child'),
          ),
        ),
      ),
    );

    // Verify child is rendered
    expect(find.text('Test Child'), findsOneWidget);

    // Verify native Tooltip widget is wrapped
    expect(find.byType(Tooltip), findsOneWidget);

    // Get the Tooltip widget and check its message
    final Tooltip tooltip = tester.widget(find.byType(Tooltip).first);
    expect(tooltip.message, contains('Your central command center'));
  });
}
