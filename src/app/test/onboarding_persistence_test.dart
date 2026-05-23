import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'dart:convert';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Restores state from backend', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft' && request.method == 'GET') {
        return http.Response(jsonEncode({
          'bio': 'Restored Bio',
          'businessName': 'Restored Bakery',
          'currentInputStep': 1, // Should jump to Name step
          'selectedTemplate': 'Classic',
        }), 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));
    await tester.pumpAndSettle();

    // Verify it jumped to step 1 (Name your business)
    expect(find.text('Name your business'), findsOneWidget);

    // Verify bio was restored (even if we are on step 1, the controller should have it)
    // Actually, it's easier to verify the name input which is on the current step
    expect(find.text('Restored Bakery'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Advanced Mode Persistence', (WidgetTester tester) async {
    final client = MockClient((request) async {
      if (request.url.path == '/api/onboarding/draft' && request.method == 'GET') {
        return http.Response(jsonEncode({
          'currentInputStep': 2, // Template step
          'isAdvancedMode': true,
          'domainChoice': 'custom',
        }), 200);
      }
      return http.Response('Not Found', 404);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(httpClient: client)));
    await tester.pumpAndSettle();

    // Verify we are on template step
    expect(find.text('Choose a look'), findsOneWidget);

    // Verify advanced mode is on and custom domain is selected
    expect(find.byKey(Key('advanced-mode-toggle')), findsOneWidget);
    final Switch advancedSwitch = tester.widget(find.byKey(Key('advanced-mode-toggle')));
    expect(advancedSwitch.value, isTrue);

    expect(find.text('Custom Domain'), findsOneWidget);
  });
}
