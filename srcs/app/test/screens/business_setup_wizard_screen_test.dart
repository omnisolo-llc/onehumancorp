import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/business_setup_wizard_screen.dart';

void main() {
  testWidgets('BusinessSetupWizardScreen renders and completes full flow', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: BusinessSetupWizardScreen(),
        ),
      ),
    );

    // Step 0: Welcome
    expect(find.text('Business Setup'), findsOneWidget);
    expect(find.text('Welcome! Your AI team, ready in minutes.'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));

    // Step 1: Company Profile
    expect(find.text('Company Profile'), findsOneWidget);
    expect(find.text('Company Name'), findsOneWidget);
    await tester.enterText(find.byType(TextFormField).at(0), 'Test Corp');

    await tester.tap(find.text('Next'));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));

    // Step 2: Select Goals
    expect(find.text('Select Goals'), findsOneWidget);
    expect(find.text('Support'), findsOneWidget);
    await tester.tap(find.text('Support')); // Toggle goal

    await tester.tap(find.text('Next'));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));

    // Step 3: Deployment Preference
    expect(find.text('Deployment Preference'), findsOneWidget);
    expect(find.text('Cloud'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));

    // Step 4: Administrator Account
    expect(find.text('Administrator Account'), findsOneWidget);
    expect(find.text('Admin Name'), findsOneWidget);
    await tester.enterText(find.byType(TextFormField).at(0), 'Admin');

    await tester.tap(find.text('Next'));
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 500));

    // Step 5: Review & Launch
    expect(find.text('Review & Launch'), findsOneWidget);
    expect(find.textContaining('Company: Test Corp'), findsOneWidget);
    expect(find.textContaining('Goals: Support'), findsOneWidget);
    expect(find.textContaining('Admin: Admin'), findsOneWidget);

    // Test Launch button
    expect(find.text('Launch My AI Team →'), findsOneWidget);
    await tester.tap(find.text('Launch My AI Team →'));
    await tester.pump(); // Start animation

    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    // Use pump instead of pumpAndSettle to avoid timeout from infinite pulsing animation
    // Wait for the mock 2s API delay
    await tester.pump(const Duration(seconds: 3));
  });
}
