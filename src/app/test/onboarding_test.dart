import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Idle State UI components present', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    expect(find.text('Let\'s build your store'), findsOneWidget);
    expect(find.byType(TextFormField), findsNWidgets(2));
    expect(find.text('Next'), findsOneWidget);
  });
}
