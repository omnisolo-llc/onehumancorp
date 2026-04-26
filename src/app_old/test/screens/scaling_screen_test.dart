import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/scaling_screen.dart';

void main() {
  testWidgets('ScalingScreen displays AI_NEWS_COLLECTOR role', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: ScalingScreen(),
          ),
        ),
      ),
    );

    // AI_NEWS_COLLECTOR should be displayed properly formatted as "AI News Collector"
    expect(find.text('AI News Collector'), findsOneWidget);
  });
}
