import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('GlassCard renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            child: const Text('Hello Glass'),
          ),
        ),
      ),
    );

    expect(find.text('Hello Glass'), findsOneWidget);
    expect(find.byType(BackdropFilter), findsOneWidget);
  });

  testWidgets('GlassCard applies margin', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: GlassCard(
            margin: const EdgeInsets.all(10),
            child: const Text('Margin Glass'),
          ),
        ),
      ),
    );

    expect(find.text('Margin Glass'), findsOneWidget);
    final paddingFinder = find.byType(Padding).first;
    expect(tester.widget<Padding>(paddingFinder).padding, const EdgeInsets.all(10));
  });
}
