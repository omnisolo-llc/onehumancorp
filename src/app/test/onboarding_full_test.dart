import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:app/main.dart';
import 'package:app/screens/onboarding.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';

void main() {
  testWidgets('Onboarding Full E2E Flow', (WidgetTester tester) async {
    // Mock the HTTP client to return 200 OK
    final mockClient = MockClient((request) async {
      return http.Response('{"success": true}', 200);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(client: mockClient)));
    await tester.pumpAndSettle();

    // Step 1: Let's build your store
    expect(find.text('Let\'s build your store'), findsOneWidget);

    // Tap Next without filling triggers validation
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    expect(find.text('Required'), findsNWidgets(2)); // Name and Category

    // Fill Step 1
    await tester.enterText(find.byType(TextFormField).first, 'My Awesome Store');
    await tester.enterText(find.byType(TextFormField).last, 'Retail');
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Final Details
    expect(find.text('Final Details'), findsOneWidget);

    // Tap Build without filling triggers validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();
    expect(find.text('Required'), findsOneWidget);

    // Fill Step 2
    await tester.enterText(find.byType(TextFormField).first, 'We sell the best retail products.');
    await tester.tap(find.text('Build My Storefront'));

    // Wait for the simulated HTTP request to complete
    await tester.pumpAndSettle();

    // Check if it reached the draft state (1-Tap Launch is visible)
    expect(find.text('1-Tap Launch'), findsOneWidget);

    // Click 1-Tap Launch
    await tester.tap(find.text('1-Tap Launch'));
    await tester.pumpAndSettle();

    // Check if it reached the StoreLiveScreen
    expect(find.text('You\'re Live!'), findsOneWidget);
  });
}
