import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders Welcome Step correctly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Initial step should be welcome screen
    expect(find.text('What are you building today?'), findsOneWidget);
    expect(find.byType(TextField), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);
  });
}
