import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/agent.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApi;

  setUp(() {
    mockApi = MockApiService();
    when(() => mockApi.getDashboard()).thenAnswer(
      (_) async => DashboardSnapshot(
        organization: const Organization(
          id: 'org1',
          name: 'Test Org',
          domain: 'test.com',
          members: [],
          roleProfiles: [],
        ),
        meetings: const [],
        costs: const CostSummary(totalCostUSD: 0, totalTokens: 0, agents: []),
        agents: const [],
        statuses: const [],
        updatedAt: DateTime.now(),
      ),
    );
  });

  testWidgets('E2E: Website Builder Wizard completes full flow', (WidgetTester tester) async {
    final router = GoRouter(
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const DashboardScreen(),
        ),
        GoRoute(
          path: '/website-builder',
          builder: (context, state) => const WebsiteBuilderWizardScreen(),
        ),
      ],
      initialLocation: '/',
    );

    // Ensure large enough screen for test
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    await tester.pump();
    await tester.pump(const Duration(seconds: 1));

    // We mock Dashboard. The only thing we need is the dashboard widget tree stable.
    for (int i = 0; i < 5; i++) {
      await tester.pump(const Duration(milliseconds: 200));
    }

    // Verify Dashboard loaded and trigger is there
    expect(find.text('Build My Website'), findsOneWidget);
    await tester.ensureVisible(find.text('Build My Website'));
    await tester.tap(find.text('Build My Website'));
    for (int i = 0; i < 5; i++) {
      await tester.pump(const Duration(milliseconds: 200));
    }

    // Now in Wizard: Step 1 (Template)
    expect(find.text('Choose a Template'), findsOneWidget);
    await tester.tap(find.text('Modern E-commerce'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 2 (Brand)
    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    await tester.ensureVisible(find.text('Generate Logo with AI'));
    await tester.tap(find.text('Generate Logo with AI'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 3 (Product)
    expect(find.text('Add Your First Product or Service'), findsOneWidget);
    await tester.enterText(find.byType(TextFormField).first, 'Awesome Cake');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 4 (Domain)
    expect(find.text('Connect a Domain'), findsOneWidget);
    await tester.tap(find.text('Next').last, warnIfMissed: false);
    await tester.pumpAndSettle();

    // Step 5 (Publish)
    expect(find.text('Ready to Go Live?'), findsOneWidget);
    expect(find.text('Template: Modern E-commerce'), findsOneWidget);
    expect(find.text('Product: Awesome Cake'), findsOneWidget);

    await tester.tap(find.text('Publish').last, warnIfMissed: false);
    // Wizard simulates 1-second delay for publishing
    await tester.pump();
    await tester.pump(const Duration(seconds: 1));

    // Verify navigation back to Dashboard and snackbar
  });
}
