import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  testWidgets('WebsiteBuilderWizardScreen renders and navigates through steps', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: WebsiteBuilderWizardScreen(),
        ),
      ),
    );

    // Initial state: Step 0 (Template)
    expect(find.text('Website Builder'), findsOneWidget);
    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    // Click a template
    await tester.tap(find.text('Portfolio'));
    await tester.pumpAndSettle();

    // Verify it was selected (showing "Use this template ->")
    expect(find.text('Use this template →'), findsOneWidget);

    // Go to Step 1 (Brand Colors & Logo)
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    expect(find.text('Upload Logo:'), findsOneWidget);

    // Go to Step 2 (First Product)
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Add your first product or service'), findsOneWidget);
    expect(find.byType(TextField), findsNWidgets(3));

    // Interact with text fields
    await tester.enterText(find.byType(TextField).first, 'Awesome Course');
    await tester.pumpAndSettle();

    // The AI description generation mockup should populate the description
    final descField = find.widgetWithText(TextField, 'Beautiful Awesome Course crafted with care.');
    expect(descField, findsOneWidget);

    // Go to Step 3 (Domain Connect)
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Connect a domain'), findsOneWidget);
    expect(find.text('Use a free OHC subdomain (mybusiness.ohc.app)'), findsOneWidget);

    // Go to Step 4 (Go Live)
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Ready to Go Live'), findsOneWidget);
    expect(find.text('Publish'), findsOneWidget);
  });

  testWidgets('WebsiteBuilderWizardScreen expert mode toggles extra fields', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: WebsiteBuilderWizardScreen(),
        ),
      ),
    );

    // Navigate to step 1
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // By default no expert mode css field
    expect(find.text('Custom CSS (Expert)'), findsNothing);

    // Toggle expert mode on
    await tester.tap(find.byType(Switch));
    await tester.pumpAndSettle();

    // Custom CSS should appear
    expect(find.text('Custom CSS (Expert)'), findsOneWidget);

    // Navigate to step 3 (Domain) to see if expert fields appear
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Connect a domain'), findsOneWidget);

    // Choose "Use my own domain" to reveal expert DNS record
    await tester.tap(find.text('Use my own domain'));
    await tester.pumpAndSettle();

    expect(find.text('Configure DNS A Records to point to 192.168.1.100'), findsOneWidget);
  });
}