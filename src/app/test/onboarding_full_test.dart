import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/start') {
        return http.Response('{"status": "ok"}', 200);
      } else if (request.url.path == '/api/onboarding/launch') {
        return http.Response('{"status": "ok"}', 200);
      } else if (request.url.path == '/api/onboarding/draft') {
        return http.Response('{"bio": "{\\"type\\": \\"Sell custom cakes\\", \\"name\\": \\"Maya\'s Cakes\\", \\"niche\\": \\"I bake custom vegan cakes\\", \\"step\\": 0}"}', 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));

    // Verify we are on Step 1
    expect(find.text('What do you do?'), findsOneWidget);

    // Enter business type
    await tester.enterText(find.byType(TextFormField).first, 'Sell custom cakes');
    await tester.pumpAndSettle();

    // Click Next
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify Step 2
    expect(find.text('What\'s the name of your business?'), findsOneWidget);

    // Enter business name
    await tester.enterText(find.byType(TextFormField).first, 'Maya\'s Cakes');
    await tester.pumpAndSettle();

    // Click Next
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Verify Step 3
    expect(find.text('What\'s your niche?'), findsOneWidget);

    // Enter niche
    await tester.enterText(find.byType(TextFormField).first, 'I bake custom vegan cakes');
    await tester.pumpAndSettle();

    // Click Generate Draft
    await tester.tap(find.text('Generate Draft'));
    await tester.pumpAndSettle();

    expect(find.text('Storefront Generated!'), findsOneWidget);

    // Go to draft preview
    await tester.tap(find.text('Preview Site'));
    await tester.pumpAndSettle();

    // Expect to be on Draft state
    expect(find.text('Looks Great!'), findsOneWidget);
    expect(find.text('Publish Now'), findsOneWidget);

    // Verify touch target for edit
    expect(find.byIcon(Icons.edit), findsOneWidget);

    // Launch store
    await tester.tap(find.text('Publish Now'));
    await tester.pumpAndSettle();

    // Expect to be on Live state
    expect(find.text("You're Live!"), findsOneWidget);
  });
}
