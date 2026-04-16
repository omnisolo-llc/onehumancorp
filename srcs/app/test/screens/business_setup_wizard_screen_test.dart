import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial state
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Step 1: Business Profile
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.byType(TextField), findsNWidgets(2)); // Company Name, Industry
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget); // Size

    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Goal selection
    expect(find.text('Select Goals'), findsOneWidget);
    expect(find.byType(CheckboxListTile), findsNWidgets(5));

    await tester.tap(find.text('Support'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Deployment Preference
    expect(find.text('Deployment Preference'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsNWidgets(3));

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Administrator account
    expect(find.byType(TextField), findsNWidgets(3)); // Admin Name, Admin Email, Admin Password

    await tester.enterText(find.byType(TextField).at(0), 'Admin');
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextField).at(2), 'password');
    await tester.pumpAndSettle();

    expect(find.text('Launch My AI Team →'), findsOneWidget);
  });
}
