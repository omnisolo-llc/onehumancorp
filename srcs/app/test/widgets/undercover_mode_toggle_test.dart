import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/undercover_mode_toggle.dart';

void main() {
  testWidgets('UndercoverModeToggle toggles state when tapped', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: UndercoverModeToggle(),
          ),
        ),
      ),
    );

    expect(find.text('Undercover Mode'), findsOneWidget);
    expect(find.byIcon(Icons.visibility), findsOneWidget);

    await tester.tap(find.text('Undercover Mode'));
    await tester.pumpAndSettle();

    expect(find.byIcon(Icons.visibility_off), findsOneWidget);
  });
}
