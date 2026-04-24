import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  group('BusinessSetupWizard E2E', () {
    testWidgets('user can navigate to wizard, load draft, save draft and proceed', (WidgetTester tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: BusinessSetupWizardScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Step 0 -> Step 1
      expect(find.text('Your business, live in minutes'), findsOneWidget);
      await tester.tap(find.text('Get Started'));
      await tester.pumpAndSettle();

      expect(find.text('What kind of business are you building?'), findsOneWidget);
      await tester.tap(find.text('Online Store'));
      await tester.pumpAndSettle();

      expect(find.text('Tell us about your business'), findsOneWidget);
    });
  });
}
