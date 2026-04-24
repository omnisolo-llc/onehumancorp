import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';

void main() {
  Widget createWidgetUnderTest() {
    final router = GoRouter(
      initialLocation: '/grow',
      routes: [
        GoRoute(
          path: '/grow',
          builder: (context, state) => const GrowMyBusinessWizardScreen(),
        ),
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const Scaffold(body: Text('Dashboard')),
        ),
      ],
    );

    return ProviderScope(
      child: MaterialApp.router(
        routerConfig: router,
      ),
    );
  }

  testWidgets('GrowMyBusinessWizardScreen renders correctly and shows options', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(createWidgetUnderTest());

    // Verify main title
    expect(find.textContaining('Grow your business'), findsOneWidget);

    // Verify options
    expect(find.text('Add 5 more products'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);
    expect(find.text('Run your first email campaign'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('GrowMyBusinessWizardScreen navigates to add products step and back', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(createWidgetUnderTest());

    await tester.tap(find.text('Add 5 more products'));
    await tester.pumpAndSettle();

    expect(find.textContaining('Ready to add more products?'), findsOneWidget);

    // Tap Go to Inventory (routes to /dashboard)
    await tester.tap(find.text('Go to Inventory'));
    await tester.pumpAndSettle();
    expect(find.text('Dashboard'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('GrowMyBusinessWizardScreen navigates to connect instagram step, mocks apply, and returns', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(createWidgetUnderTest());

    await tester.tap(find.text('Connect Instagram'));
    await tester.pumpAndSettle();

    expect(find.textContaining('Connect your Instagram Professional account'), findsOneWidget);

    // Tap Back
    await tester.tap(find.text('Back'));
    await tester.pumpAndSettle();
    expect(find.textContaining('Grow your business'), findsOneWidget);

    await tester.tap(find.text('Connect Instagram'));
    await tester.pumpAndSettle();

    // Tap Connect
    await tester.tap(find.text('Connect Account'));
    await tester.pump();

    // Show loading
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle(const Duration(seconds: 2));

    expect(find.text('Instagram Connected!'), findsOneWidget);

    await tester.tap(find.text('Return to Dashboard'));
    await tester.pumpAndSettle();
    expect(find.text('Dashboard'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });

  testWidgets('GrowMyBusinessWizardScreen navigates to email campaign and back', (WidgetTester tester) async {
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    await tester.pumpWidget(createWidgetUnderTest());

    await tester.tap(find.text('Run your first email campaign'));
    await tester.pumpAndSettle();

    expect(find.textContaining('Let The Promoter draft an email'), findsOneWidget);

    // Tap Back
    await tester.tap(find.text('Back'));
    await tester.pumpAndSettle();
    expect(find.textContaining('Grow your business'), findsOneWidget);

    await tester.tap(find.text('Run your first email campaign'));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Start Drafting'));
    await tester.pumpAndSettle();
    expect(find.text('Dashboard'), findsOneWidget);

    tester.view.resetPhysicalSize();
    tester.view.resetDevicePixelRatio();
  });
}
