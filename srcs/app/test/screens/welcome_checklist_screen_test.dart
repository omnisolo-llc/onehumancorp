import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/welcome_checklist_screen.dart';

void main() {
  testWidgets('WelcomeChecklistScreen renders tasks and toggles them', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(
      child: MaterialApp(
        home: WelcomeChecklistScreen(),
      ),
    ));

    expect(find.text("You're set up! Here's what to do next:"), findsOneWidget);
    expect(find.text('Business Live'), findsOneWidget);
    expect(find.text('Add 3 more products'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);
    expect(find.text('Share your link with a friend'), findsOneWidget);
    expect(find.text('Go to my Dashboard ->'), findsOneWidget);

    // Initial state: Add 3 more products is not crossed out
    final productsText = tester.widget<Text>(find.text('Add 3 more products'));
    expect(productsText.style?.decoration, isNot(TextDecoration.lineThrough));

    // Tap 'Add 3 more products'
    await tester.tap(find.text('Add 3 more products'));
    await tester.pumpAndSettle();

    // After tap, it should be crossed out
    final productsTextAfter = tester.widget<Text>(find.text('Add 3 more products'));
    expect(productsTextAfter.style?.decoration, TextDecoration.lineThrough);
  });
}
