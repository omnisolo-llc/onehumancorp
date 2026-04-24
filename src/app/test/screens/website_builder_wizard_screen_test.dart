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
          builder: (context, state) => const Scaffold(body: WebsiteBuilderWizardScreen()),
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
    await tester.pumpAndSettle();

    expect(find.text('Website Builder Work in Progress'), findsNothing);
    // Initial state
    expect(find.text('Select a template to start with.'), findsOneWidget);

    // Step 1: Select Template
    await tester.tap(find.text('Use this template →').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Proceed to Step 2
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 2: Brand
    expect(find.text('Pick your brand color palette.'), findsOneWidget);
    await tester.ensureVisible(find.text('Blue/Gold').first);
    await tester.tap(find.text('Blue/Gold').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Proceed to Step 3
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 3: Product
    expect(find.byType(TextField), findsNWidgets(3));
    await tester.enterText(find.byType(TextField).at(0), 'Test Product');
    await tester.pumpAndSettle();

    // Proceed to Step 4
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 4: Domain
    expect(find.text('Progressive Disclosure Mode'), findsOneWidget);
    await tester.ensureVisible(find.text('Use a free OHC subdomain (mybusiness.ohc.app)').first);
    await tester.tap(find.text('Use a free OHC subdomain (mybusiness.ohc.app)').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Proceed to Step 5
    await tester.ensureVisible(find.text('Next').first);
    await tester.tap(find.text('Next').first, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 5: Go Live
    expect(find.text('Preview your live site!'), findsOneWidget);
    expect(find.text('Publish'), findsOneWidget);
  });

  test('WebsiteBuilderNotifier state transitions', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(websiteBuilderProvider.notifier);

    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.nextStep();
    expect(container.read(websiteBuilderProvider).step, 1);

    notifier.prevStep();
    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.updateTemplate('Modern E-commerce');
    expect(container.read(websiteBuilderProvider).selectedTemplate, 'Modern E-commerce');

    notifier.updateBrandColor('Blue/Gold');
    expect(container.read(websiteBuilderProvider).brandColor, 'Blue/Gold');

    notifier.updateLogoUrl('my_logo.png');
    expect(container.read(websiteBuilderProvider).logoUrl, 'my_logo.png');

    notifier.updateProductName('Super Product');
    expect(container.read(websiteBuilderProvider).productName, 'Super Product');

    notifier.updateProductPrice('19.99');
    expect(container.read(websiteBuilderProvider).productPrice, '19.99');

    notifier.generateAIDescription();
    expect(container.read(websiteBuilderProvider).productDescription, 'AI generated description for Super Product');

    notifier.updateDomainChoice('own');
    expect(container.read(websiteBuilderProvider).domainChoice, 'own');

    notifier.updateExpertMode(true);
    expect(container.read(websiteBuilderProvider).expertMode, true);
  });
}
