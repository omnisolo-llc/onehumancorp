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
    await tester.pump();

    // Tap next to step 2
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Goal Selection'), findsOneWidget);

    // Tap next to step 3
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Deployment Preference'), findsOneWidget);

    // Tap next to step 4
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Admin Name'), findsOneWidget);

        // Tap next to step 5
    await tester.tap(find.text('Next'));
    await tester.pump();
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Launch My AI Team'), findsOneWidget);
  });
}
