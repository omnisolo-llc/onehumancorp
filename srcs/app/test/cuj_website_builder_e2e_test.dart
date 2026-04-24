import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/website_builder_onboarding_screen.dart';

void main() {
  testWidgets('WebsiteBuilderOnboardingScreen renders first step correctly', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const WebsiteBuilderOnboardingScreen(),
        ),
      ],
    );

    // Set a physical size to prevent flex overflow
    tester.view.physicalSize = const Size(1000, 2000);
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
    await tester.pumpAndSettle();

    expect(find.text('Choose a Template'), findsOneWidget);
    expect(find.text('Modern Retail'), findsOneWidget);
    expect(find.text('Service Booking'), findsOneWidget);
  });

  testWidgets('WebsiteBuilderOnboardingScreen handles full wizard flow', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const WebsiteBuilderOnboardingScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Text('Dashboard'),
        ),
      ],
    );

    // Set a physical size to prevent flex overflow
    tester.view.physicalSize = const Size(1000, 2000);
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
    await tester.pumpAndSettle();

    // Step 0: Templates
    await tester.tap(find.text('Modern Retail'));
    await tester.pumpAndSettle();
    await tester.ensureVisible(find.text('Next'));
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 1: Brand
    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    await tester.tap(find.text('Generate'));
    await tester.pumpAndSettle();
    expect(find.text('Logo ready!'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 2: Offering
    expect(find.text('Add your first offering'), findsOneWidget);
    await tester.enterText(find.byType(TextField).at(0), 'Signature Cake');
    await tester.enterText(find.byType(TextField).at(1), '45');
    // Mock the auto-awesome tap for description
    await tester.tap(find.byIcon(Icons.auto_awesome));
    await tester.pumpAndSettle();
    // Simulate photo upload
    await tester.tap(find.text('Upload Photo'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 3: Domain
    expect(find.text('Connect a domain'), findsOneWidget);
    await tester.tap(find.text('Use a free OHC subdomain'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    // Step 4: Publish
    expect(find.text('Ready to go live!'), findsOneWidget);
    await tester.tap(find.text('Publish'));
    await tester.pump(const Duration(seconds: 2));
    await tester.pumpAndSettle();

    expect(find.text('Dashboard'), findsOneWidget);
  });
}
