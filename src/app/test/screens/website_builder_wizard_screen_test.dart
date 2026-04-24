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

    // Initial state: Step 0
    expect(find.text('Website Builder'), findsOneWidget);
    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Next'), findsOneWidget);

    await tester.tap(find.text('Minimal'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1
    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    await tester.tap(find.text('Upload Logo'));
    await tester.pumpAndSettle();
    expect(find.text('Selected: uploaded_logo.png'), findsOneWidget);

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2
    expect(find.text('Add your first product or service'), findsOneWidget);
    expect(find.byType(TextField), findsNWidgets(3));

    await tester.enterText(find.byType(TextField).first, 'Test Product');
    await tester.enterText(find.byType(TextField).at(1), '19.99');
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3
    expect(find.text('Connect a domain'), findsOneWidget);
    expect(find.byType(RadioListTile<String>), findsNWidgets(2));

    await tester.tap(find.text('Use my own domain'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4
    expect(find.text('Ready to go live?'), findsOneWidget);
    expect(find.text('Publish'), findsOneWidget);

    await tester.tap(find.text('Publish'));
    await tester.pump(); // Start animation
    await tester.pump(const Duration(seconds: 2)); // Wait for simulated delay
    await tester.pumpAndSettle();

    // Navigate to dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });

  test('WebsiteBuilderNotifier state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(websiteBuilderProvider.notifier);

    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.nextStep();
    expect(container.read(websiteBuilderProvider).step, 1);

    notifier.prevStep();
    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.prevStep();
    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.selectTemplate('Modern');
    expect(container.read(websiteBuilderProvider).template, 'Modern');

    notifier.updateColor('#33FF57');
    expect(container.read(websiteBuilderProvider).primaryColor, '#33FF57');

    notifier.updateLogo('test.png');
    expect(container.read(websiteBuilderProvider).logoUrl, 'test.png');

    notifier.updateProductName('Prod');
    expect(container.read(websiteBuilderProvider).productName, 'Prod');

    notifier.updateProductPrice(10.5);
    expect(container.read(websiteBuilderProvider).productPrice, 10.5);

    notifier.updateProductDescription('Desc');
    expect(container.read(websiteBuilderProvider).productDescription, 'Desc');

    notifier.updateDomainPreference('custom_domain');
    expect(container.read(websiteBuilderProvider).domainPreference, 'custom_domain');
  });
}
