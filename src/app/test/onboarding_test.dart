import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    expect(find.text('OneHumanCorp'), findsOneWidget);
    expect(find.text('The universal operating system for small business.'), findsOneWidget);
    expect(find.text('Start a Business'), findsOneWidget);

    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('Welcome to OHC Smart Builder'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Shows SnackBar on API failure', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{}', 200);
      }
      if (request.url.path == '/api/onboarding/intake') {
        return http.Response('{}', 500);
      }
      return http.Response('Internal Server Error', 500);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Go to input state
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Enter text
    await tester.enterText(find.byKey(Key('bio-input')), 'Test business bio');
    await tester.pumpAndSettle();

    // Submit form
    // Let's use tester.tap finding by type instead
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pumpAndSettle();

    // Expect to see SnackBar
    expect(find.text('Network error. Please try again.'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Transitions to Generating state on form submit', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{}', 200);
      }
      if (request.url.path == '/api/onboarding/intake') {
        return http.Response('{"business_name": "Test"}', 200);
      }
      if (request.url.path == '/api/onboarding/start') {
        // Delay response to allow verifying Generating state
        await Future.delayed(Duration(milliseconds: 500));
        return http.Response('{"status": "ok"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Go to input state
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Enter text
    await tester.enterText(find.byKey(Key('bio-input')), 'Test generating state');
    await tester.pumpAndSettle();

    // Submit form
    await tester.tap(find.byType(ElevatedButton).last);
    await tester.pump(); // Don't pump and settle, so we can see the generating state!

    // Verify generating state UI
    expect(find.text('AI is building your storefront...'), findsOneWidget);
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle(const Duration(seconds: 1));
  });

  testWidgets('Onboarding Screen - Loads existing draft bio on init', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft' && request.method == 'GET') {
        return http.Response('{"bio": "Existing loaded bio"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));
    await tester.pumpAndSettle();

    // Go to input state
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Verify the text field has the loaded bio
    expect(find.text('Existing loaded bio'), findsOneWidget);
    await tester.pumpAndSettle(const Duration(seconds: 1));
  });
}
