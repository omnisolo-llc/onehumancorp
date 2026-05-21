import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'dart:convert';

void main() {
  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    final mockClient = MockClient((request) async {
      return http.Response(jsonEncode({'step': 0, 'bio': ''}), 200);
    });

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen(client: mockClient)));

    expect(find.text('OneHumanCorp'), findsOneWidget);
    expect(find.text('The universal operating system for small business.'), findsOneWidget);
    expect(find.text('Start a Business'), findsOneWidget);
  });
}
