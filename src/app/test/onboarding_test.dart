import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Welcome State UI components present', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    expect(find.text('What are you building today?'), findsOneWidget);
    expect(find.text('Let AI set up your business in seconds.'), findsOneWidget);
    expect(find.text('Get Started'), findsOneWidget);
  });
}
