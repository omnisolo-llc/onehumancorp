import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders initial welcome step', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(body: BusinessSetupWizardScreen()),
        ),
      ),
    );

    expect(find.text('Your business, live in minutes.'), findsOneWidget);
    expect(find.text('Get Started'), findsOneWidget);
  });

  testWidgets('BusinessSetupWizardScreen can navigate steps', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: Scaffold(body: BusinessSetupWizardScreen()),
        ),
      ),
    );

    // Initial state step 0
    expect(find.text('Your business, live in minutes.'), findsOneWidget);

    // Tap Get Started
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();

    // Step 1: Business Type
    expect(find.text('What kind of business are you building?'), findsOneWidget);
    expect(find.text('Online Store'), findsOneWidget);

    // Tap a business type. In the new wizard, tapping the tile auto-advances.
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();

    // Step 2: Name & Desc
    expect(find.text('Name & Description'), findsOneWidget);

    // Enter Name to advance
    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.ensureVisible(find.widgetWithText(FilledButton, 'Next'));
    await tester.tap(find.widgetWithText(FilledButton, 'Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // Step 3: What do you sell
    expect(find.text('What do you sell?'), findsOneWidget);

    // Tap Physical products
    await tester.tap(find.text('Physical products'));
    await tester.pump(const Duration(milliseconds: 500));
    await tester.ensureVisible(find.widgetWithText(FilledButton, 'Next'));
    await tester.tap(find.widgetWithText(FilledButton, 'Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // Step 4: Payments
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    await tester.tap(find.text('Online only'));
    await tester.pump(const Duration(milliseconds: 500));

    // Step 5: Admin Account
    expect(find.text('Create your Administrator account'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Admin User');
    await tester.enterText(find.byType(TextField).at(1), 'admin@ohc.local');
    await tester.enterText(find.byType(TextField).at(2), 'Pass123!');
    await tester.ensureVisible(find.widgetWithText(FilledButton, 'Next'));
    await tester.tap(find.widgetWithText(FilledButton, 'Next'));
    await tester.pump(const Duration(milliseconds: 500));

    // Step 6: Review Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Launch My Business →'), findsOneWidget);
  });
}
