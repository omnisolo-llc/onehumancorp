import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  testWidgets('WebsiteBuilderWizardScreen renders and navigates steps', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const WebsiteBuilderWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard')),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    // Initial state: Step 0 (Template gallery)
    expect(find.text('Build My Website'), findsOneWidget);
    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Modern Minimal'), findsOneWidget);
    expect(find.text('Bold Storefront'), findsOneWidget);
    expect(find.text('Creative Portfolio'), findsOneWidget);

    // Select a template
    await tester.tap(find.text('Modern Minimal'));
    await tester.pumpAndSettle();

    // Template selection shows "Use this template" button
    expect(find.text('Use this template →'), findsOneWidget);
    await tester.tap(find.text('Use this template →'));
    await tester.pumpAndSettle();

    // Step 1: Brand Colors & Logo
    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    expect(find.byType(ChoiceChip), findsNWidgets(3));
    await tester.tap(find.text('Ocean'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Generate a logo for me'));
    await tester.pumpAndSettle();
    expect(find.text('Logo selected: auto-generated-logo.png'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Add product
    expect(find.text('Add your first product or service'), findsOneWidget);
    final textFields = find.byType(TextField);
    expect(textFields, findsNWidgets(3));

    await tester.enterText(textFields.at(0), 'My Awesome Product');
    await tester.pumpAndSettle();
    await tester.enterText(textFields.at(1), '19.99');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Connect a domain
    expect(find.text('Connect a domain'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsNWidgets(3));

    // Test Back button
    await tester.tap(find.text('Back'));
    await tester.pumpAndSettle();
    expect(find.text('Add your first product or service'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Go Live
    expect(find.text('Preview & Go Live'), findsOneWidget);
    expect(find.text('Template: Modern Minimal'), findsOneWidget);
    expect(find.text('Palette: Ocean'), findsOneWidget);
    expect(find.text('Product: My Awesome Product - \$19.99'), findsOneWidget);
    expect(find.text('Domain: Free Subdomain'), findsOneWidget);

    await tester.tap(find.text('Publish'));
    await tester.pump(); // Start loading
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle(const Duration(seconds: 2)); // Wait for simulated network delay

    // Routing to dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });

  test('WebsiteBuilderNotifier state changes', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(websiteBuilderProvider.notifier);

    notifier.selectTemplate('Test Template');
    expect(container.read(websiteBuilderProvider).selectedTemplate, 'Test Template');

    notifier.selectPalette('Test Palette');
    expect(container.read(websiteBuilderProvider).selectedPalette, 'Test Palette');

    notifier.updateLogo('test-logo.png');
    expect(container.read(websiteBuilderProvider).logoUrl, 'test-logo.png');

    notifier.updateProductName('Test Product');
    expect(container.read(websiteBuilderProvider).productName, 'Test Product');
    expect(container.read(websiteBuilderProvider).productDesc, 'A wonderful Test Product for your needs.');

    notifier.updateProductPrice('100');
    expect(container.read(websiteBuilderProvider).productPrice, '100');

    notifier.updateProductDesc('Custom description');
    expect(container.read(websiteBuilderProvider).productDesc, 'Custom description');

    notifier.selectDomain('custom');
    expect(container.read(websiteBuilderProvider).domainChoice, 'custom');
  });
}
