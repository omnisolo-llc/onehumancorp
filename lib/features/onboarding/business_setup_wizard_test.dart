import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'business_setup_wizard.dart';

void main() {
  testWidgets('BusinessSetupWizard end-to-end happy path', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: BusinessSetupWizard(),
          ),
        ),
      ),
    );
    expect(find.byType(Stepper), findsOneWidget);
    expect(find.text('Welcome to OHC'), findsOneWidget);

    // Welcome -> Step 2
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();

    // Step 2 Validation Error
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();
    expect(find.byType(SnackBar), findsOneWidget);

    // Fill Step 2
    await tester.enterText(find.byType(TextFormField).first, 'Test Company');
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();

    // Step 3 -> Step 4
    expect(find.text('What are your goals?'), findsOneWidget);
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();

    // Step 4 -> Step 5
    expect(find.text('Deployment Preference'), findsOneWidget);
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();

    // Step 5
    expect(find.text('Administrator Account'), findsOneWidget);

    // Validation
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();


    // Fill Step 5
    await tester.enterText(find.byType(TextFormField).at(0), 'Admin User');
    await tester.enterText(find.byType(TextFormField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextFormField).at(2), 'secure_password_123');
    await tester.tap(find.text('Next').first);
    await tester.pumpAndSettle();

    // Step 6 (Review)


  });
}
