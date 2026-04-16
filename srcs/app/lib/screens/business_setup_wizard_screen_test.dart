import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders', (tester) async {
    await tester.pumpWidget(
      const ProviderScope(child: MaterialApp(home: BusinessSetupWizardScreen())),
    );
    await tester.pumpAndSettle();

    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
  });
}
