import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/main.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:ohc_app/services/settings_service.dart';

class _LoginAuthNotifier extends AuthNotifier {
  @override
  Future<AuthSession?> build() async => null;

  @override
  Future<void> login(String email, String password) async {
    state = const AsyncData(AuthSession(
      user: AuthUser(id: '1', email: 'test@example.com'),
      token: 'fake-token',
    ));
  }

  @override
  Future<void> logout() async {
    state = const AsyncData(null);
  }
}

void main() {
  testWidgets('CUJ: Business Setup Wizard flow', (WidgetTester tester) async {
    final overrides = [
      authStateProvider.notifier.overrideWith(() => _LoginAuthNotifier()),
      backendUrlProvider.overrideWithValue('http://localhost:18789'),
    ];

    await tester.pumpWidget(ProviderScope(overrides: overrides, child: const OhcApp()));
    await tester.pumpAndSettle();

    // The user lands on LandingScreen, clicks Login
    expect(find.text('Login'), findsOneWidget);
    await tester.tap(find.text('Login'));
    await tester.pumpAndSettle();

    // Now on LoginScreen
    expect(find.text('Sign in to OHC'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'test@example.com');
    await tester.enterText(find.byType(TextField).last, 'password123');
    await tester.tap(find.text('Sign In'));
    await tester.pumpAndSettle();

    // Now on Dashboard, find "Setup Wizard" in sidebar
    expect(find.text('Setup Wizard'), findsOneWidget);
    await tester.tap(find.text('Setup Wizard'));
    await tester.pumpAndSettle();

    // Step 0: Welcome screen
    expect(find.text('Your business, live in minutes.'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Business type
    expect(find.text('What type of business are you starting?'), findsOneWidget);
    await tester.tap(find.text('Online Store'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Business name & description
    expect(find.text('Tell us about your business'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'My Awesome Store');
    await tester.enterText(find.byType(TextField).last, 'Selling awesome things');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: What do you sell?
    expect(find.text('What do you sell?'), findsOneWidget);
    await tester.tap(find.text('Physical products'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payment method
    expect(find.text('How do you want to receive payments?'), findsOneWidget);
    await tester.tap(find.text('Online only'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 5: Administrator account
    expect(find.text('Administrator Account'), findsOneWidget);
    await tester.enterText(find.widgetWithText(TextField, 'Name'), 'Admin Admin');
    await tester.enterText(find.widgetWithText(TextField, 'Email'), 'admin@example.com');
    await tester.enterText(find.widgetWithText(TextField, 'Password'), 'supersecret123');
    await tester.pumpAndSettle();

    // Check strength indicator
    expect(find.text('Strong'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 6: Review & Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('My Awesome Store'), findsOneWidget);
    expect(find.text('Online Store'), findsOneWidget);
    expect(find.text('Physical products'), findsOneWidget);
    expect(find.text('Online only'), findsOneWidget);
    expect(find.text('admin@example.com'), findsOneWidget);

    // Launch!
    // Since we don't have a real backend, we mock the network or just verify the button exists and tap it.
    // The button will fail network, but we verify it's there.
    expect(find.text('Launch My Business →'), findsOneWidget);
  });
}
