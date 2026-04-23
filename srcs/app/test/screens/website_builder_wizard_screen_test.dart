import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  testWidgets('WebsiteBuilderWizardScreen full flow with one-tap domain', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(
        child: MaterialApp(
          home: WebsiteBuilderWizardScreen(),
        ),
      ),
    );

    // Step 1: Template
    expect(find.text('Step 1: Choose a Template'), findsOneWidget);
    await tester.tap(find.text('Minimalist'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Use this template →'));
    await tester.pumpAndSettle();

    // Step 2: Brand
    expect(find.text('Step 2: Brand Colors & Logo'), findsOneWidget);
    await tester.tap(find.text('Ocean Blue'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next Step →'));
    await tester.pumpAndSettle();

    // Step 3: Product
    expect(find.text('Step 3: Add your first product or service'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'My Cake');
    await tester.tap(find.text('Next Step →'));
    await tester.pumpAndSettle();

    // Step 4: Domain
    expect(find.text('Step 4: Connect a domain'), findsOneWidget);
    // Tap the free subdomain and verify it goes to Step 5 immediately
    await tester.tap(find.text('Use a free OHC subdomain (mybusiness.ohc.app)'));
    await tester.pumpAndSettle();

    // Step 5: Publish
    expect(find.text('Step 5: Go Live'), findsOneWidget);
    await tester.tap(find.text('Publish'));
    await tester.pump(); // trigger loading
    expect(find.byType(CircularProgressIndicator), findsOneWidget);
    await tester.pump(const Duration(seconds: 3)); // finish loading
  });
}
