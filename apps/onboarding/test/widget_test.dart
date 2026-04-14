import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:onboarding/main.dart';

void main() {
  testWidgets('OnboardingScreen renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const OnboardingApp());

    expect(find.textContaining('Welcome to One Human Corp'), findsOneWidget);
    expect(find.textContaining('Setup Complete'), findsOneWidget);
  });
}
