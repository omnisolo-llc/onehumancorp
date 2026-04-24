import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  Widget createWidgetUnderTest() {
    final router = GoRouter(
      initialLocation: '/website-builder',
      routes: [
        GoRoute(
          path: '/website-builder',
          builder: (context, state) => const WebsiteBuilderWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard Mock')),
        ),
      ],
    );

    return ProviderScope(
      child: MaterialApp.router(
        routerConfig: router,
      ),
    );
  }

  testWidgets('Website builder wizard step transitions work correctly', (WidgetTester tester) async {
    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    // Verify initial step (Template Gallery)
    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Minimal'), findsOneWidget);

    // Initial state: Use template button should be disabled (cannot tap)
    final templateNextButton = find.widgetWithText(ElevatedButton, 'Use this template →');
    expect(tester.widget<ElevatedButton>(templateNextButton).enabled, isFalse);

    // Select a template
    await tester.tap(find.text('Minimal'));
    await tester.pumpAndSettle();

    // Button should now be enabled
    expect(tester.widget<ElevatedButton>(templateNextButton).enabled, isTrue);

    // Go to Step 1 (Brand Colors & Logo)
    await tester.tap(templateNextButton);
    await tester.pumpAndSettle();

    expect(find.text('Brand Identity'), findsOneWidget);

    // Next button should be disabled
    final brandNextButton = find.widgetWithText(ElevatedButton, 'Next Step →');
    expect(tester.widget<ElevatedButton>(brandNextButton).enabled, isFalse);

    // Select color palette 1 (blue) and toggle AI logo
    await tester.tap(find.byType(GestureDetector).first); // Select palette1
    await tester.pumpAndSettle();

    await tester.tap(find.text('Generate a logo for me (AI)'));
    await tester.pumpAndSettle();

    // Next button should be enabled
    expect(tester.widget<ElevatedButton>(brandNextButton).enabled, isTrue);

    // Go to Step 2 (Add Product/Service)
    await tester.tap(brandNextButton);
    await tester.pumpAndSettle();

    expect(find.text('First Product'), findsOneWidget);

    // Next button should be disabled initially
    final productNextButton = find.widgetWithText(ElevatedButton, 'Next Step →');
    expect(tester.widget<ElevatedButton>(productNextButton).enabled, isFalse);

    // Enter product details
    await tester.enterText(find.byType(TextField).at(0), 'Test Cake');
    await tester.enterText(find.byType(TextField).at(1), '20.00');
    await tester.pumpAndSettle();

    // Test AI description button
    await tester.tap(find.text('AI Write'));
    await tester.pumpAndSettle(const Duration(seconds: 2)); // wait for mocked AI response

    // Now next should be enabled
    expect(tester.widget<ElevatedButton>(productNextButton).enabled, isTrue);

    // Go to Step 3 (Connect Domain)
    await tester.tap(productNextButton);
    await tester.pumpAndSettle();

    expect(find.text('Connect a Domain'), findsOneWidget);

    // Next button should be disabled initially
    final domainNextButton = find.widgetWithText(ElevatedButton, 'Review & Publish →');
    expect(tester.widget<ElevatedButton>(domainNextButton).enabled, isFalse);

    // Select OHC subdomain
    await tester.tap(find.text('Use a free OHC subdomain'));
    await tester.pumpAndSettle();

    expect(tester.widget<ElevatedButton>(domainNextButton).enabled, isTrue);

    // Go to Step 4 (Go Live Preview)
    await tester.tap(domainNextButton);
    await tester.pumpAndSettle();

    expect(find.text('Ready to Launch'), findsOneWidget);
    expect(find.text('Test Cake'), findsOneWidget); // Make sure name propagated

    // Publish
    await tester.tap(find.text('Publish Now 🚀'));
    await tester.pump(); // start delay
    await tester.pumpAndSettle(const Duration(seconds: 3)); // wait for publish delay and navigation

    // Should navigate to dashboard
    expect(find.text('Dashboard Mock'), findsOneWidget);
  });
}
