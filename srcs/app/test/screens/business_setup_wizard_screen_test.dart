import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen navigates steps and updates state', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial state (Step 0)
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team is ready to be configured in minutes.'), findsOneWidget);

    // Tap next to go to Step 1
    await tester.tap(find.text('Next Step'));
    await tester.pumpAndSettle();

    expect(find.text('Business Profile'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Test Company');
    await tester.enterText(find.byType(TextField).at(1), 'Technology');

    // Tap next to go to Step 2
    await tester.tap(find.text('Next Step'));
    await tester.pumpAndSettle();

    expect(find.text('Select Goals'), findsOneWidget);
    await tester.tap(find.text('Support'));
    await tester.pumpAndSettle();

    // Tap next to go to Step 3
    await tester.tap(find.text('Next Step'));
    await tester.pumpAndSettle();

    expect(find.text('Deployment Preference'), findsOneWidget);
    await tester.tap(find.text('Desktop'));
    await tester.pumpAndSettle();

    // Tap next to go to Step 4
    await tester.tap(find.text('Next Step'));
    await tester.pumpAndSettle();

    expect(find.text('Administrator Account'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Admin User');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'securepassword');

    // Tap next to go to Step 5
    await tester.tap(find.text('Next Step'));
    await tester.pumpAndSettle();

    // Final review step
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Test Company'), findsOneWidget);
    expect(find.text('Desktop'), findsOneWidget);
    expect(find.text('admin@test.com'), findsOneWidget);

    // Final button should be visible
    expect(find.text('Launch My AI Team →'), findsOneWidget);
  });
}
