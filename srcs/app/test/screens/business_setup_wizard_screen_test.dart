import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../lib/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen steps through wizard completely', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Step 0
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1
    expect(find.byType(TextField).first, findsOneWidget); // Company
    expect(find.text('Company Name'), findsOneWidget);
    expect(find.text('Industry'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'Test Co');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2
    expect(find.text('Select Goals'), findsOneWidget);
    await tester.tap(find.text('Support'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3
    expect(find.text('Deployment Preference'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4
    expect(find.text('Admin Name'), findsOneWidget);
    expect(find.text('Admin Email'), findsOneWidget);
    expect(find.text('Admin Password'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Company: Test Co'), findsOneWidget);
    expect(find.text('Launch My AI Team'), findsOneWidget);
  });
}
