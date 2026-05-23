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
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect to see SnackBar
    expect(find.text('Network error. Please try again.'), findsOneWidget);

    // Expect to remain in input state since it errors out
    expect(find.text('Welcome to OHC Smart Builder'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Debounce triggers draft save', (WidgetTester tester) async {
    bool draftSaved = false;
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft' && request.method == 'POST') {
        draftSaved = true;
        return http.Response('{"status": "ok"}', 200);
      }
      if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Go to input state
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Enter text (this should trigger debounce)
    await tester.enterText(find.byKey(Key('bio-input')), 'Test business bio for debounce');

    // Wait for debounce timer (500ms)
    await tester.pump(Duration(milliseconds: 600));

    expect(draftSaved, isTrue);
  });

  testWidgets('Onboarding Screen - Transitions to Generating state on form submit', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{}', 200);
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
    await tester.tap(find.text('Build My Storefront'));
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