import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/screens/onboarding.dart';

void main() {
  testWidgets('Onboarding Screen - Full Flow Test', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    expect(find.text('Your Details'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    expect(find.text('Required'), findsOneWidget);

    await tester.enterText(find.byType(TextFormField).first, "Maya's Custom Cakes");

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Your First Product'), findsOneWidget);

    final textFields = find.byType(TextFormField);
    await tester.enterText(textFields.at(0), "Custom Cake Deposit");
    await tester.enterText(textFields.at(1), "25.00");
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Select Template'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Attach Domain'), findsOneWidget);

    await tester.tap(find.text('Build My Storefront'));
    await tester.pumpAndSettle();
  });
}
