import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders child', (WidgetTester tester) async {
    const text = 'Glass Card Content';
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            child: const Text(text),
          ),
        ),
      ),
    );

    expect(find.text(text), findsOneWidget);
    expect(find.byType(GlassCard), findsOneWidget);
  });

  testWidgets('GlassCard handles tap', (WidgetTester tester) async {
    bool tapped = false;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            onTap: () {
              tapped = true;
            },
            child: const Text('Tap Me'),
          ),
        ),
      ),
    );

    await tester.tap(find.byType(GlassCard));
    await tester.pumpAndSettle();

    expect(tapped, isTrue);
  });
}
