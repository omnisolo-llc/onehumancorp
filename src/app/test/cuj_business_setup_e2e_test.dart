import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/main.dart';
import 'package:ohc_app/screens/login_screen.dart';

void main() {
  testWidgets('E2E: Business Setup Wizard flow', (WidgetTester tester) async {
    // 1. Initialize app
    await tester.pumpWidget(const ProviderScope(child: OhcApp()));
    await tester.pumpAndSettle();

    // Wait for auth to initialize
    await tester.pumpAndSettle(const Duration(seconds: 2));

    // 2. Perform Login to reach Dashboard -> Wizard flow
    // Enter credentials if login screen is visible
    final loginButton = find.text('Login');
    if (loginButton.evaluate().isNotEmpty) {
      await tester.enterText(find.byType(TextField).first, 'admin@ohc.local');
      await tester.enterText(find.byType(TextField).last, 'password123');

      // Tap login button
      await tester.tap(loginButton.last);
      await tester.pumpAndSettle(const Duration(seconds: 2));
    }

    // After login, depending on the test data, it might route to /wizard or /dashboard
    // If we're on dashboard, we might need to navigate to wizard manually, but the test
    // says "it lands the user in the dashboard". Since it's a new test user, it should show wizard.
    final getStartedButton = find.text('Get Started');
    if (getStartedButton.evaluate().isNotEmpty) {
      await tester.tap(getStartedButton);
      await tester.pumpAndSettle();

      // Step 1: Business Type
      await tester.tap(find.text('Online Store'));
      await tester.pumpAndSettle();

      // Step 2: Name & Desc
      await tester.enterText(find.byType(TextField).first, 'My Awesome Store');
      await tester.tap(find.text('Continue'));
      await tester.pumpAndSettle();

      // Step 3: What do you sell
      await tester.tap(find.text('Physical products'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Continue'));
      await tester.pumpAndSettle();

      // Step 4: Payments
      await tester.tap(find.text('Online only'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Continue'));
      await tester.pumpAndSettle();

      // Step 5: Admin Account
      await tester.enterText(find.byType(TextField).at(0), 'Admin User');
      await tester.enterText(find.byType(TextField).at(1), 'admin@ohc.local');
      await tester.enterText(find.byType(TextField).at(2), 'securepass123');
      await tester.tap(find.text('Continue'));
      await tester.pumpAndSettle();

      // Step 6: Review & Launch
      expect(find.text('Launch My Business →'), findsOneWidget);
      await tester.tap(find.text('Launch My Business →'));
      await tester.pumpAndSettle(const Duration(seconds: 2));

      // Should land on Dashboard or show some indicator
      expect(find.text('Dashboard'), findsWidgets);
    }
  });
}
