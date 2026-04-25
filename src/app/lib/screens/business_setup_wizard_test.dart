import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen navigation', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: BusinessSetupWizardScreen())));

    // Step 0 -> 1
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();
    expect(find.text('What kind of business are you building?'), findsOneWidget);

    // Step 1 -> 2
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();
    expect(find.text('Tell us about your business'), findsOneWidget);

    // Step 2 -> 3
    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('What do you sell?'), findsOneWidget);

    // Step 3 -> 4
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('How do you want to receive payments?'), findsOneWidget);

    // Step 4 -> 5
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();
    expect(find.text('Administrator account'), findsOneWidget);

    // Step 5 -> 6
    await tester.enterText(find.byType(TextField).at(0), 'Admin');
    await tester.enterText(find.byType(TextField).at(1), 'admin@example.com');
    await tester.enterText(find.byType(TextField).at(2), 'password');
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();
    expect(find.text('Review & Launch'), findsOneWidget);

    // Step 6 Launch
    // await tester.tap(find.text('Launch My Business →')); // requires GoRouter in context
    await tester.pumpAndSettle();
  });
}
