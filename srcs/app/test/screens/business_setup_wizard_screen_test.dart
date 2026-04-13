import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and completes flow', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Company Name'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'Test Company');

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Select Goals'), findsOneWidget);
    await tester.tap(find.text('Support'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Deployment Preference'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Admin Name'), findsOneWidget);
    await tester.enterText(find.widgetWithText(TextField, 'Admin Name'), 'Admin');

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pump();
  });
}
