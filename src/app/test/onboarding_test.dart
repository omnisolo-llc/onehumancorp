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

  testWidgets('Onboarding Screen - Step wizard navigation test', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    await tester.tap(find.text('Start a Business'));
    await tester.pumpAndSettle();

    expect(find.text('Your Details'), findsOneWidget);

    await tester.enterText(find.byType(TextFormField).first, 'Test Business');

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Your First Product'), findsOneWidget);

    await tester.tap(find.byIcon(Icons.arrow_back));
    await tester.pumpAndSettle();

    expect(find.text('Your Details'), findsOneWidget);
  });

  testWidgets('Onboarding Screen - Responsiveness UI sizes appropriately Desktop', (WidgetTester tester) async {
    tester.view.physicalSize = Size(1440, 900);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    final containerFinder = find.byType(Container).first;
    expect(tester.getSize(containerFinder).width, equals(768.0));
  });

  testWidgets('Onboarding Screen - Responsiveness UI sizes appropriately Mobile', (WidgetTester tester) async {
    tester.view.physicalSize = Size(375, 812);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);

    await tester.pumpWidget(MaterialApp(home: OnboardingScreen()));

    final containerFinder = find.byType(Container).first;
    expect(tester.getSize(containerFinder).width, equals(375.0));
  });
}
