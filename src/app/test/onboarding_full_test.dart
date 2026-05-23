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

  testWidgets('Onboarding Screen - Full 4-Step Flow Test', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/start') {
        await Future.delayed(Duration(milliseconds: 500));
        return http.Response('{"status": "ok"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Welcome Screen
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Step 1: Business Info
    expect(find.text('Step 1 of 4'), findsOneWidget);

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    expect(find.text('Required'), findsWidgets);

    await tester.enterText(find.byKey(Key('business-name-input')), 'Test Business Name');
    await tester.enterText(find.byKey(Key('business-category-input')), 'Test Category');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Visual Style
    expect(find.text('Step 2 of 4'), findsOneWidget);

    // Try without selecting
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    expect(find.text('Please select a visual style'), findsOneWidget);

    // Select style
    await tester.ensureVisible(find.text('Elegant'));
    await tester.tap(find.text('Elegant'));
    await tester.pumpAndSettle();

    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: First Item
    expect(find.text('Step 3 of 4'), findsOneWidget);

    await tester.enterText(find.byKey(Key('item-name-input')), 'First Awesome Item');
    await tester.enterText(find.byKey(Key('item-price-input')), '42.00');
    await tester.pumpAndSettle();

    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Payment
    expect(find.text('Step 4 of 4'), findsOneWidget);

    // Initial state check
    expect(find.text('Connect Bank Account'), findsOneWidget);

    await tester.tap(find.text('Connect Bank Account'));
    await tester.pumpAndSettle();

    expect(find.text('Bank Connected'), findsOneWidget);

    // Launch
    await tester.tap(find.text('Launch My Business'));
    await tester.pump(); // Don't pumpAndSettle to catch loading state if needed, but going to pumpAndSettle is okay since API is mocked fast

    // Expect generating
    expect(find.text('AI is building your storefront...'), findsOneWidget);

    await tester.pumpAndSettle();

    // Final Success / Live state
    expect(find.text('Store Live!'), findsOneWidget);
    expect(find.text('https://testbusinessname.ohc.app'), findsOneWidget);
    expect(find.text('Go to Dashboard'), findsOneWidget);
  });
}
