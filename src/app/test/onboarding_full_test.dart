import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'dart:convert';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    final mockClient = MockClient((request) async {
      if (request.method == 'GET' && request.url.path == '/api/onboarding/state') {
        return http.Response(jsonEncode({'step': 0, 'bio': ''}), 200);
      } else if (request.method == 'POST') {
        return http.Response('', 200);
      }
      return http.Response('', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(client: mockClient)));

    // Wait for the initState to finish
    await tester.pumpAndSettle();

    // Tap 'Start a Business' on the welcome screen
    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    // Verify we are on the input screen
    expect(find.text('Welcome to OHC Smart Builder'), findsOneWidget);

    // Tap without entering text to trigger validation
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // Expect the validator message 'Required' for the bio field
    expect(find.text('Required'), findsOneWidget);

    // Fill out the bio form
    await tester.enterText(find.byKey(Key('bio-input')), "I bake custom vegan cakes in Seattle. Maya's Cakes.");
    await tester.pumpAndSettle();

    // Submit form correctly and await for mock network processing.
    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();

    // We expect state transitioned to dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });
}
