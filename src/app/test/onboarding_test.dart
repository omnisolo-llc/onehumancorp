import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../lib/screens/onboarding.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    expect(find.text('OneHumanCorp'), findsOneWidget);
    expect(find.text('The universal operating system for small business.'), findsOneWidget);
    expect(find.text('Start a Business'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Shows SnackBar on API failure in step 4', (WidgetTester tester) async {
    final client = MockClient((request) async {
      return http.Response('Internal Server Error', 500);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Go to step 1
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Step 1: Business Info
    await tester.enterText(find.byKey(Key('business-name-input')), 'Test Business');
    await tester.enterText(find.byKey(Key('business-category-input')), 'Bakery');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Visual Style
    await tester.ensureVisible(find.text('Elegant'));
    await tester.tap(find.text('Elegant'));
    await tester.pumpAndSettle();

    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: First Item
    await tester.enterText(find.byKey(Key('item-name-input')), 'Test Item');
    await tester.enterText(find.byKey(Key('item-price-input')), '10.0');
    await tester.pumpAndSettle();

    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payment Connect
    await tester.tap(find.text('Connect Bank Account'));
    await tester.pumpAndSettle();

    // Submit form
    await tester.tap(find.text('Launch My Business'));
    await tester.pumpAndSettle();

    // Expect to see SnackBar
    expect(find.text('Network error. Please try again.'), findsOneWidget);

    // Expect to still be on step 4
    expect(find.text('Step 4 of 4'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Persistent state across steps using SharedPreferences', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({
      'businessName': 'Existing Business',
      'businessCategory': 'Testing',
      'onboardingState': OnboardingState.step2.index,
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: MockClient((r) async => http.Response('{}', 200)))));
    await tester.pumpAndSettle();

    // Should load directly to step 2 based on saved prefs
    expect(find.text('Step 2 of 4'), findsOneWidget);
  });
}
