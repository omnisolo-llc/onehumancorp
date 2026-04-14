import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/mode_indicator.dart';

void main() {
  testWidgets('ModeIndicator renders Cloud Mode by default', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: ModeIndicator(),
          ),
        ),
      ),
    );

    expect(find.text('Cloud Mode'), findsOneWidget);
    expect(find.byIcon(Icons.cloud), findsOneWidget);
  });
}
