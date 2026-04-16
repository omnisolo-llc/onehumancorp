import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen validates company name on step 1', (WidgetTester tester) async {
    // Provide a physical size to avoid overflow during animations.
    tester.view.physicalSize = const Size(2400, 1600);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial state (Step 0)
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    // Tap Next to go to Step 1
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify we are on Step 1
    expect(find.text('Company Name'), findsOneWidget);

    // Try to tap Next without filling company name
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify error message is shown and we are still on Step 1
    expect(find.text('Company Name is required.'), findsOneWidget);
    expect(find.text('Company Name'), findsOneWidget);

    // Enter company name
    await tester.enterText(find.widgetWithText(TextField, 'Company Name'), 'Test Company');
    await tester.pumpAndSettle();

    // Tap Next
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify error message goes away and we reach Step 2
    expect(find.text('Company Name is required.'), findsNothing);
    expect(find.text('Select Goals'), findsOneWidget);
  });
}
