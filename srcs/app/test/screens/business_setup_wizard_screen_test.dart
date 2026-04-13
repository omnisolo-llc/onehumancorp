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

    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    // Tap next
    await tester.tap(find.text('Next'));
    await tester.pump();

    expect(find.text('Company Name'), findsOneWidget);
    await tester.enterText(find.widgetWithText(TextField, 'Company Name'), 'Acme Corp');
    await tester.enterText(find.widgetWithText(TextField, 'Industry'), 'Tech');
    await tester.pump();

    // Tap next to step 2
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Select Goals'), findsOneWidget);
    await tester.tap(find.text('Support'));
    await tester.pump();

    // Tap next to step 3
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Deployment Preference'), findsOneWidget);
    await tester.tap(find.text('Desktop'));
    await tester.pump();

    // Tap next to step 4
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Admin Name'), findsOneWidget);
    await tester.enterText(find.widgetWithText(TextField, 'Admin Name'), 'Admin');
    await tester.enterText(find.widgetWithText(TextField, 'Admin Email'), 'admin@example.com');
    await tester.enterText(find.widgetWithText(TextField, 'Admin Password'), 'password123');
    await tester.pump();

    expect(find.text('Launch My AI Team →'), findsOneWidget);
  });
}
