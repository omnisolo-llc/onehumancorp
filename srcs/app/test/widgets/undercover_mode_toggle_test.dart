import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/undercover_mode_toggle.dart';

void main() {
  testWidgets('UndercoverModeToggle toggles state correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: UndercoverModeToggle(),
          ),
        ),
      ),
    );

    // Initial state should be false
    expect(find.text('Undercover Mode'), findsOneWidget);
    expect(find.byIcon(Icons.visibility), findsOneWidget);

    final switchFinder = find.byType(Switch);
    expect(switchFinder, findsOneWidget);

    Switch switchWidget = tester.widget(switchFinder);
    expect(switchWidget.value, isFalse);

    // Tap the switch
    await tester.tap(switchFinder);
    await tester.pumpAndSettle();

    // New state should be true
    expect(find.byIcon(Icons.visibility_off), findsOneWidget);
    switchWidget = tester.widget(switchFinder);
    expect(switchWidget.value, isTrue);
  });
}
