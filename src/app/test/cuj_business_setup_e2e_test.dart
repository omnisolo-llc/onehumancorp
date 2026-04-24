import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('CUJ: Business Setup Wizard UI flow', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const BusinessSetupWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard Loaded')),
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

    // Step 0 -> 1
    expect(find.text('Your business, live in minutes'), findsOneWidget);
    await tester.tap(find.text('Get Started'));
    await tester.pumpAndSettle();

    // Step 1 -> 2
    expect(find.text('What kind of business are you building?'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();

    // Step 2 -> 3
    expect(find.text('Tell us about your business'), findsOneWidget);
    await tester.enterText(find.widgetWithText(TextField, 'Business Name'), 'My Test Shop');
    await tester.enterText(find.widgetWithText(TextField, 'Short Description'), 'A shop for tests');
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 3 -> 4
    expect(find.text('What do you sell?'), findsOneWidget);
    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 4 -> 5
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 5 -> 6
    expect(find.text('Administrator account'), findsOneWidget);
    await tester.enterText(find.widgetWithText(TextField, 'Name'), 'Admin User');
    await tester.enterText(find.widgetWithText(TextField, 'Email'), 'admin@testshop.com');
    await tester.enterText(find.widgetWithText(TextField, 'Password'), 'securepassword');
    await tester.tap(find.text('Continue'));
    await tester.pumpAndSettle();

    // Step 6 -> Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('My Test Shop'), findsOneWidget);
    expect(find.text('Online Store'), findsOneWidget);
    expect(find.text('Physical products'), findsOneWidget);
    expect(find.text('Online only'), findsOneWidget);
    expect(find.text('admin@testshop.com'), findsOneWidget);

    await tester.tap(find.text('Launch My Business →'));
    await tester.pumpAndSettle();

    // Since we didn't mock http or auth, the API call will fail/bypass with 'Not authenticated'
    // Let's assert we handled it properly, or that we didn't crash.
    expect(find.text('Not authenticated'), findsOneWidget);
  });
}
