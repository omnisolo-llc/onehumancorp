import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';
import 'package:go_router/go_router.dart';

void main() {
  testWidgets('WebsiteBuilderWizardScreen navigates through all steps', (WidgetTester tester) async {
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

    // Initial state (Step 0)
    expect(find.text('Build My Website'), findsOneWidget);
    expect(find.text('Choose a template'), findsOneWidget);

    // Cannot proceed without selecting a template
    final nextButtonFinder = find.widgetWithText(ElevatedButton, 'Next');
    expect(tester.widget<ElevatedButton>(nextButtonFinder).enabled, isFalse);

    // Select a template
    await tester.tap(find.text('Portfolio'));
    await tester.pumpAndSettle();

    final useTemplateFinder = find.widgetWithText(ElevatedButton, 'Use this template →');
    expect(tester.widget<ElevatedButton>(useTemplateFinder).enabled, isTrue);

    // Go to Step 1
    await tester.tap(useTemplateFinder);
    await tester.pumpAndSettle();
    expect(find.text('Brand Colors & Logo'), findsOneWidget);

    // Go to Step 2
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();
    expect(find.text('Add your first product or service'), findsOneWidget);

    // Check advanced mode toggle
    final switchFinder = find.byType(Switch);
    expect(switchFinder, findsWidgets); // could be multiple switches on screen
    await tester.tap(switchFinder.first);
    await tester.pumpAndSettle();

    // Enter product details
    await tester.enterText(find.byType(TextField).at(0), 'Consulting');
    await tester.enterText(find.byType(TextField).at(1), '100');
    await tester.enterText(find.byType(TextField).at(2), 'Expert advice');
    await tester.pumpAndSettle();

    // Back and forward test
    await tester.ensureVisible(find.widgetWithText(OutlinedButton, 'Back'));
    await tester.tap(find.widgetWithText(OutlinedButton, 'Back'));
    await tester.pumpAndSettle();
    expect(find.text('Brand Colors & Logo'), findsOneWidget);

    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();

    // Go to Step 3
    await tester.ensureVisible(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();
    expect(find.text('Connect a domain'), findsOneWidget);

    // Select custom domain
    await tester.tap(find.text('Use my own domain'));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), 'mycustomdomain.com');
    await tester.pumpAndSettle();

    // Go to Step 4
    await tester.tap(find.widgetWithText(ElevatedButton, 'Next'));
    await tester.pumpAndSettle();
    expect(find.text('Go Live: Review your site'), findsOneWidget);
    expect(find.text('Domain: mycustomdomain.com'), findsOneWidget);

    // Publish
    await tester.tap(find.text('Publish'));
    await tester.pumpAndSettle();

    // Check navigation to dashboard
    expect(find.text('Dashboard'), findsOneWidget);
  });

  test('WebsiteBuilderNotifier state mutations', () {
    final container = ProviderContainer();
    addTearDown(container.dispose);

    final notifier = container.read(websiteBuilderProvider.notifier);
    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.nextStep();
    expect(container.read(websiteBuilderProvider).step, 1);

    notifier.previousStep();
    expect(container.read(websiteBuilderProvider).step, 0);

    notifier.updateTemplate('E-commerce');
    expect(container.read(websiteBuilderProvider).selectedTemplate, 'E-commerce');

    notifier.updateColor('#123456');
    expect(container.read(websiteBuilderProvider).primaryColor, '#123456');

    notifier.updateProduct('A', '10', 'Desc');
    expect(container.read(websiteBuilderProvider).productName, 'A');
    expect(container.read(websiteBuilderProvider).productPrice, '10');
    expect(container.read(websiteBuilderProvider).productDescription, 'Desc');

    notifier.updateDomain('custom', 'test.com');
    expect(container.read(websiteBuilderProvider).domainChoice, 'custom');
    expect(container.read(websiteBuilderProvider).customDomain, 'test.com');
  });
}
