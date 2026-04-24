import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/material.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/services/auth_service.dart';

void main() {
  group('BusinessSetupWizardScreen Tests', () {
    testWidgets('BusinessSetupWizardScreen renders and navigates steps', (WidgetTester tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            backendUrlProvider.overrideWithValue('http://localhost'),
          ],
          child: const MaterialApp(
            home: BusinessSetupWizardScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Step 0 check
      expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

      // Tap Next
      final nextFinder = find.widgetWithText(ElevatedButton, 'Next');
      await tester.tap(nextFinder);
      await tester.pumpAndSettle();

      // Step 1 check
      expect(find.text('Business type'), findsOneWidget);

      // Provide a valid value and go to next

      await tester.tap(nextFinder);
      await tester.pumpAndSettle();

      // Step 2
      expect(find.text('Business name'), findsWidgets);

      // Test Back button
      final backFinder = find.widgetWithText(TextButton, 'Back');
      await tester.tap(backFinder);
      await tester.pumpAndSettle();

      expect(find.text('Business type'), findsOneWidget);
    });

    test('BusinessSetupNotifier initial state is step 0', () {
      final container = ProviderContainer();
      final state = container.read(businessSetupProvider);
      expect(state.step, 0);
      container.dispose();
    });
  });
}
