import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:go_router/go_router.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and navigates steps with validation', (WidgetTester tester) async {
    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard')),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    // Initial state
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Step 1: Navigate to Business Profile
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.byType(TextField), findsNWidgets(2)); // Company Name, Industry
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget); // Size

    // Attempt to navigate without input to trigger validation
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    expect(find.text('Please fill in all fields.'), findsOneWidget);

    // Fill valid input
    await tester.enterText(find.byType(TextField).first, 'Test Company');
    await tester.enterText(find.byType(TextField).last, 'Tech');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Goal selection
    expect(find.text('Select Goals'), findsOneWidget);
    expect(find.byType(CheckboxListTile), findsNWidgets(5));

    // Attempt to navigate without selecting goals
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    expect(find.text('Please select at least one goal.'), findsOneWidget);

    // Select goal
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

    // Attempt to launch without input
    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();
    expect(find.text('Please fill in all fields.'), findsOneWidget);

    // Fill Name and Invalid Email
    await tester.enterText(find.byType(TextField).at(0), 'Admin');
    await tester.enterText(find.byType(TextField).at(1), 'invalid_email');
    await tester.enterText(find.byType(TextField).at(2), 'pass');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();
    expect(find.text('Please enter a valid email address.'), findsOneWidget);

    // Fill valid email but invalid password
    await tester.enterText(find.byType(TextField).at(1), 'admin@test.com');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();
    expect(find.text('Password must be at least 8 characters long.'), findsOneWidget);

    // Fill valid password
    await tester.enterText(find.byType(TextField).at(2), 'password123');
    await tester.pumpAndSettle();

    // Now it should launch (or attempt to, showing loading state if mock backend was present, but here it just clears errors)
    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pumpAndSettle();

    // The error message should be gone
    expect(find.text('Password must be at least 8 characters long.'), findsNothing);
  });
}
