import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders and responds to hover', (WidgetTester tester) async {
    bool tapped = false;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            onTap: () {
              tapped = true;
            },
            child: const Text('Glass Card Content'),
          ),
        ),
      ),
    );

    expect(find.text('Glass Card Content'), findsOneWidget);

    await tester.tap(find.byType(GlassCard));
    expect(tapped, isTrue);
  });
}
