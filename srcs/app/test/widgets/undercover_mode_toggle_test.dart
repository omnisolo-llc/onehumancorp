import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/undercover_mode_toggle.dart';

void main() {
  testWidgets('UndercoverModeToggle renders and toggles state correctly', (WidgetTester tester) async {
    final container = ProviderContainer();

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: const MaterialApp(
          home: Scaffold(
            body: UndercoverModeToggle(),
          ),
        ),
      ),
    );

    // Initial state check
    expect(find.text('Undercover Mode'), findsOneWidget);
    expect(find.byIcon(Icons.visibility), findsOneWidget);
    expect(container.read(undercoverModeProvider), false);

    // Tap to toggle on
    await tester.tap(find.byType(Switch));
    await tester.pumpAndSettle();

    // Checked state
    expect(find.byIcon(Icons.visibility_off), findsOneWidget);
    expect(container.read(undercoverModeProvider), true);

    // Tap to toggle off
    await tester.tap(find.byType(Switch));
    await tester.pumpAndSettle();

    // Unchecked state
    expect(find.byIcon(Icons.visibility), findsOneWidget);
    expect(container.read(undercoverModeProvider), false);
  });
}
