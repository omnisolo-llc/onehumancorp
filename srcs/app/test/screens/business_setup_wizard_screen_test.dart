import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps using Stepper', (WidgetTester tester) async {

    tester.view.physicalSize = const Size(1200, 1600);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(() => tester.view.resetPhysicalSize());
    addTearDown(() => tester.view.resetDevicePixelRatio());

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp(
          home: Scaffold(
            body: Container(
              child: BusinessSetupWizardScreen(),
            ),
          ),
        ),
      ),
    );

    // Give it a moment to render
    await tester.pumpAndSettle();

    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    // Tap the 'Next' button on the first step
    final nextButtons = find.text('Next');
    await tester.ensureVisible(nextButtons.first);
    await tester.tap(nextButtons.first);
    await tester.pumpAndSettle();

    expect(find.byType(TextField), findsWidgets); // Company Name, Industry
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget); // Size

    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.pumpAndSettle();

    await tester.ensureVisible(nextButtons.at(1));
    await tester.tap(nextButtons.at(1));
    await tester.pumpAndSettle();

    // Step 2: Goal selection
    expect(find.text('Select Goals'), findsOneWidget);
    expect(find.byType(CheckboxListTile), findsWidgets);

    await tester.tap(find.text('Support').last);
    await tester.pumpAndSettle();

    await tester.ensureVisible(nextButtons.at(2));
    await tester.tap(nextButtons.at(2));
    await tester.pumpAndSettle();

    // Step 3: Deployment Preference
    expect(find.text('Deployment Preference'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsWidgets);

    await tester.ensureVisible(nextButtons.at(3));
    await tester.tap(nextButtons.at(3));
    await tester.pumpAndSettle();

    // Step 4: Administrator account
    final textFields = find.byType(TextField);
    await tester.enterText(textFields.at(textFields.evaluate().length - 3), 'Admin');
    await tester.enterText(textFields.at(textFields.evaluate().length - 2), 'admin@test.com');
    await tester.enterText(textFields.at(textFields.evaluate().length - 1), 'password');
    await tester.pumpAndSettle();

    final launchButton = find.text('Launch My AI Team →');
    await tester.ensureVisible(launchButton.first);
    expect(launchButton, findsWidgets);
  });
}
