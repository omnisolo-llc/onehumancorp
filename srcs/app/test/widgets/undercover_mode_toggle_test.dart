import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/widgets/undercover_mode_toggle.dart';

void main() {
  testWidgets('UndercoverModeToggle toggles state when pressed', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: UndercoverModeToggle(),
          ),
        ),
      ),
    );

    // Initial state (false)
    expect(find.byIcon(Icons.visibility), findsOneWidget);
    expect(find.byIcon(Icons.visibility_off), findsNothing);

    // Tap to toggle
    await tester.tap(find.byType(UndercoverModeToggle));
    await tester.pumpAndSettle();

    // New state (true)
    expect(find.byIcon(Icons.visibility), findsNothing);
    expect(find.byIcon(Icons.visibility_off), findsOneWidget);
  });
}
