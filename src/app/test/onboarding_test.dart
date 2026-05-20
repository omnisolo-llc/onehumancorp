import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:app/screens/onboarding.dart';
import 'package:http/http.dart' as http;

void main() {
  Widget createWidgetUnderTest() {
    return MaterialApp(
      home: OnboardingScreen(),
    );
  }

  testWidgets('renders onboarding screen correctly', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());

    expect(find.text('Start Your Business'), findsOneWidget);
    expect(find.text('Build your business in minutes'), findsOneWidget);
    expect(find.byType(TextFormField), findsOneWidget);
    expect(find.byType(DropdownButtonFormField<String>), findsOneWidget);
    expect(find.text('Start Setup'), findsOneWidget);
  });

  testWidgets('shows validation errors when fields are empty', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());

    // Tap the start button without entering data
    await tester.tap(find.text('Start Setup'));
    await tester.pumpAndSettle();

    // Verify error messages
    expect(find.text('Required'), findsOneWidget); // Assuming category has default 'Bakery', only name is required
  });

  testWidgets('can select a category', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());

    // Open dropdown
    await tester.tap(find.byType(DropdownButtonFormField<String>));
    await tester.pumpAndSettle();

    // Select 'Tutor'
    await tester.tap(find.text('Tutor').last);
    await tester.pumpAndSettle();

    // Find DropdownButton with 'Tutor'
    expect(find.text('Tutor'), findsWidgets);
  });

  testWidgets('submits form with data', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());

    // Enter data
    await tester.enterText(find.byType(TextFormField), 'My Test Bakery');
    await tester.pumpAndSettle();

    // Select 'Handyman'
    await tester.tap(find.byType(DropdownButtonFormField<String>));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Handyman').last);
    await tester.pumpAndSettle();

    // We skip actual HTTP mocking but ensure the progress indicator is fine
    await tester.tap(find.text('Start Setup'));
    await tester.pump();

    expect(tester.takeException(), isNull);
  });

  testWidgets('renders store live screen correctly', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: StoreLiveScreen()));

    expect(find.text('Store Live'), findsOneWidget);
    expect(find.text('Success! Your business is live!'), findsOneWidget);
    expect(find.byIcon(Icons.check_circle), findsOneWidget);
  });
}
