import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:app/screens/business_setup_wizard_screen.dart';

void main() {
  Widget createWidgetUnderTest() {
    return const ProviderScope(
      child: MaterialApp(
        home: BusinessSetupWizardScreen(),
      ),
    );
  }

  testWidgets('Test 1: Welcome screen renders and navigates to next step', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());

    // Verify Welcome step
    expect(find.text('Welcome to One Human Corp'), findsOneWidget);
    expect(find.text('Get Started'), findsOneWidget);

    // Tap Get Started
    await tester.tap(find.text('Get Started'));
    await tester.pump(const Duration(seconds: 1));

    // Verify next step is Business Profile
    expect(find.text('Business Profile'), findsOneWidget);
  });

  testWidgets('Test 2: Business Profile accepts input and navigates to next step', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    tester.view.physicalSize = const Size(800, 1200);
    tester.view.devicePixelRatio = 1.0;

    // Navigate to step 1
    await tester.tap(find.text('Get Started'));
    await tester.pump(const Duration(seconds: 1));

    // Verify step 1
    expect(find.text('Business Profile'), findsOneWidget);

    // Enter details
    await tester.enterText(find.byType(TextFormField).at(0), 'Test Company');
    await tester.enterText(find.byType(TextFormField).at(1), 'Tech');
    await tester.enterText(find.byType(TextFormField).at(2), '1-10');

    // Tap Next
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify next step is Goal Selection
    expect(find.text('What are your primary goals?'), findsOneWidget);
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });

  testWidgets('Test 3: Goal Selection accepts input and navigates to next step', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    tester.view.physicalSize = const Size(800, 1200);
    tester.view.devicePixelRatio = 1.0;

    // Navigate to step 2
    await tester.tap(find.text('Get Started'), warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.enterText(find.byType(TextFormField).at(0), 'Test Company');
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify step 2
    expect(find.text('What are your primary goals?'), findsOneWidget);

    // Select goals
    await tester.tap(find.text('Build Software').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.tap(find.text('Customer Support').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Tap Next
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify next step is Deployment
    expect(find.text('Deployment Preference'), findsOneWidget);
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });

  testWidgets('Test 4: Deployment Preference accepts input and navigates to next step', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    tester.view.physicalSize = const Size(800, 1200);
    tester.view.devicePixelRatio = 1.0;

    // Navigate to step 3
    await tester.tap(find.text('Get Started'), warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.enterText(find.byType(TextFormField).at(0), 'Test Company');
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify step 3
    expect(find.text('Deployment Preference'), findsOneWidget);

    // Select Deployment
    await tester.tap(find.text('Cloud').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Tap Next
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify next step is Admin Account
    expect(find.text('Administrator Account'), findsOneWidget);
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });

  testWidgets('Test 5: Administrator Account, Review & Launch flow completes successfully', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    tester.view.physicalSize = const Size(800, 1200);
    tester.view.devicePixelRatio = 1.0;

    // Navigate to step 4
    await tester.tap(find.text('Get Started'), warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.enterText(find.byType(TextFormField).at(0), 'Test Company');
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.tap(find.text('Build Software').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.tap(find.text('Cloud').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify step 4
    expect(find.text('Administrator Account'), findsOneWidget);

    // Enter details
    await tester.enterText(find.byType(TextFormField).at(0), 'Admin User');
    await tester.enterText(find.byType(TextFormField).at(1), 'admin@test.com');
    await tester.enterText(find.byType(TextFormField).at(2), 'password123');

    // Tap Next
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 1));

    // Verify step 5 (Review)
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.text('Test Company'), findsWidgets);
    expect(find.text('build'), findsWidgets);
    expect(find.text('cloud'), findsWidgets);
    expect(find.text('admin@test.com'), findsWidgets);

    // Tap Launch
    await tester.tap(find.text('Launch My AI Team').last, warnIfMissed: false);
    await tester.pump(); // Trigger launch

    // Check loading indicator
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Let the timer finish
    await tester.pump(const Duration(seconds: 2));
    await tester.pump(const Duration(seconds: 1));

    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
