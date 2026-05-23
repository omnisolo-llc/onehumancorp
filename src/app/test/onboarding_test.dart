import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    // The default state is now input. To test welcome we would need to simulate state change
    // but the UI currently starts in Input state directly. We just test if Input UI is present.
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    expect(find.text('What do you do?'), findsOneWidget);
    expect(find.text('Step 1 of 3'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Shows SnackBar on API failure', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{}', 200);
      }
      return http.Response('Internal Server Error', 500);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Already in input state
    await tester.pumpAndSettle();

    // Enter text
    await tester.enterText(find.byKey(Key('type-input')), 'Test business type');
    await tester.pumpAndSettle();

    // Next to Name step
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Enter name
    await tester.enterText(find.byKey(Key('name-input')), 'My Test Business');
    await tester.pumpAndSettle();

    // Next to Niche step
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.enterText(find.byKey(Key('niche-input')), 'My Niche');
    await tester.pumpAndSettle();

    // Submit form
    await tester.tap(find.text('Generate Draft'));
    await tester.pumpAndSettle();

    // Expect to see SnackBar
    expect(find.text('Network error. Please try again.'), findsOneWidget);

    // Expect to still be on the Niche selection state (after going back to input state on error)
    // Actually the code says setState(() => _state = OnboardingState.input);
    // So it stays in the Input state but the _currentInputStep will remain what it was (2).
    expect(find.text('What\'s your niche?'), findsOneWidget);
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

    // Already in input state
    await tester.pumpAndSettle();

    // Enter text (this should trigger debounce)
    await tester.enterText(find.byKey(Key('type-input')), 'Test business type for debounce');

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

    // Already in input state
    await tester.pumpAndSettle();

    // Enter text
    await tester.enterText(find.byKey(Key('type-input')), 'Test generating state');
    await tester.pumpAndSettle();

    // Next to Name step
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Enter name
    await tester.enterText(find.byKey(Key('name-input')), 'My Test Business');
    await tester.pumpAndSettle();

    // Next to Niche step
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.enterText(find.byKey(Key('niche-input')), 'My Niche');
    await tester.pumpAndSettle();

    // Submit form
    await tester.tap(find.text('Generate Draft'));
    await tester.pump(); // Don't pump and settle, so we can see the generating state!

    // Verify generating state UI
    expect(find.text('AI is building your storefront...'), findsOneWidget);
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle(const Duration(seconds: 1));
  });

  testWidgets('Onboarding Screen - Loads existing draft bio on init', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft' && request.method == 'GET') {
        return http.Response('{"businessType": "Existing loaded type"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));
    await tester.pumpAndSettle();

    // Verify the text field has the loaded bio
    expect(find.text('Existing loaded type'), findsOneWidget);
    await tester.pumpAndSettle(const Duration(seconds: 1));
  });
}
