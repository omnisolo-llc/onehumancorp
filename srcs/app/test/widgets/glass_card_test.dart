import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  group('GlassCard Widget Tests', () {
    testWidgets('renders correctly with child', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: GlassCard(
              child: const Text('Test Content'),
            ),
          ),
        ),
      );

      expect(find.byType(GlassCard), findsOneWidget);
      expect(find.text('Test Content'), findsOneWidget);
    });

    testWidgets('responds to hover events', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: GlassCard(
              child: const Text('Hover Me'),
            ),
          ),
        ),
      );

      final glassCardFinder = find.byType(GlassCard);
      expect(glassCardFinder, findsOneWidget);

      final MouseRegion mouseRegion = tester.widget(find.descendant(
        of: glassCardFinder,
        matching: find.byType(MouseRegion),
      ));

      expect(mouseRegion.onEnter, isNotNull);
      expect(mouseRegion.onExit, isNotNull);
    });
  });
}
