import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';

void main() {
  testWidgets('WebsiteBuilderWizardScreen renders all steps', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const Scaffold(body: Text('Home')),
        ),
        GoRoute(
          path: '/website-builder',
          builder: (context, state) => const WebsiteBuilderWizardScreen(),
        ),
      ],
      initialLocation: '/website-builder',
    );

    // Ensure large enough screen for test
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );
    await tester.pump();
    await tester.pump(const Duration(seconds: 1));

    // Step 0: Template Gallery
    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Modern E-commerce'), findsOneWidget);
    await tester.tap(find.text('Modern E-commerce'));
    await tester.pumpAndSettle();

    expect(find.text('Next'), findsOneWidget);
    await tester.tap(find.text('Next'), warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 1: Brand Colors
    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    expect(find.text('Generate Logo with AI'), findsOneWidget);
    await tester.ensureVisible(find.text('Generate Logo with AI'));
    await tester.tap(find.text('Generate Logo with AI'));
    await tester.pumpAndSettle();
    expect(find.text('Logo selected (AI Generated)'), findsOneWidget);

    await tester.tap(find.text('Next'), warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 2: Product Service
    expect(find.text('Add Your First Product or Service'), findsOneWidget);
    await tester.enterText(find.byType(TextFormField).first, 'Super Widget');
    await tester.pumpAndSettle();
    expect(find.text('Auto-generate Description'), findsOneWidget);
    await tester.ensureVisible(find.text('Auto-generate Description'));
    await tester.tap(find.text('Auto-generate Description'));
    await tester.pumpAndSettle();

    // Ensure widget has time to build state
    await tester.pumpAndSettle();
    await tester.pump(const Duration(milliseconds: 500));
    // Check state update
    final container = ProviderScope.containerOf(tester.element(find.byType(WebsiteBuilderWizardScreen)));
    expect(container.read(websiteBuilderProvider).productDescription, contains('beautifully crafted Super Widget'));

    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 3: Domain
    expect(find.text('Connect a Domain'), findsOneWidget);
    expect(find.text('Use a free OHC subdomain'), findsOneWidget);

    await tester.tap(find.text('Next'), warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 4: Publish
    expect(find.text('Ready to Go Live?'), findsOneWidget);
    expect(find.text('Template: Modern E-commerce'), findsOneWidget);

    await tester.tap(find.text('Publish').last, warnIfMissed: false);
    await tester.pump(const Duration(seconds: 2));
    await tester.pumpAndSettle();

    // SnackBar should have appeared and router navigated to /
    expect(find.text('Home'), findsOneWidget);
  });
}
