import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    expect(find.text('OneHumanCorp'), findsOneWidget);
    expect(find.text('The universal operating system for small business.'), findsOneWidget);
    expect(find.text('Start a Business'), findsOneWidget);
  });
}
