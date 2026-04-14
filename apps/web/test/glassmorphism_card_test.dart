import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/widgets/glassmorphism_card.dart';

void main() {
  testWidgets('GlassmorphismCard renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: GlassmorphismCard(
            child: Text('Glassmorphism Text'),
          ),
        ),
      ),
    );

    expect(find.byType(GlassmorphismCard), findsOneWidget);
    expect(find.text('Glassmorphism Text'), findsOneWidget);
  });
}
