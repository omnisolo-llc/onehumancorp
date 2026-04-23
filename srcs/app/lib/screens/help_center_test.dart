import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/services/tooltip_registry.dart';
import 'package:ohc_app/screens/help_center_screen.dart';

void main() {
  testWidgets('E2E Help Center Flow: Help Center -> Search -> Tooltip & FAB', (tester) async {
    // Provide a fresh TooltipRegistry
    final container = ProviderContainer();

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: const MaterialApp(
          home: HelpCenterScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    // 4. Verify Help Center Screen
    expect(find.text('How can we help you?'), findsOneWidget);
    // Verify an article from the provider is shown
    expect(find.text('Set up your store'), findsOneWidget);

    // 5. Search for an article
    await tester.enterText(find.byType(TextField), 'Instagram');
    await tester.pumpAndSettle();

    // Now only the Instagram article should be shown
    expect(find.text('How to sell on Instagram'), findsOneWidget);
    expect(find.text('Set up your store'), findsNothing);

    // Clear search
    await tester.enterText(find.byType(TextField), '');
    await tester.pumpAndSettle();
    expect(find.text('Set up your store'), findsOneWidget);

  });
}
