import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/website_builder_wizard_screen.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/services/api_service.dart';

class FakeApiService implements ApiService {
  @override
  Future<DashboardSnapshot> getDashboard() async {
    return DashboardSnapshot(
      mrr: 1000.0,
      activeUsers: 500,
      activeAgents: 5,
      tasksCompleted: 1500,
      queueLength: 0,
      activeMissions: [],
      idleAgents: [],
      totalCost: 10.0,
      totalCredits: 100.0,
      systemHealth: 'HEALTHY',
    );
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}


void main() {
  testWidgets('CUJ: Website Builder Wizard completes full flow', (WidgetTester tester) async {
    final fakeApi = FakeApiService();

    final router = GoRouter(
      initialLocation: '/dashboard',
      routes: [
        GoRoute(
          path: '/dashboard',
          builder: (context, state) => const DashboardScreen(),
        ),
        GoRoute(
          path: '/wizard/website',
          builder: (context, state) => const WebsiteBuilderWizardScreen(),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(fakeApi),
        ],
        child: MaterialApp.router(
          routerConfig: router,
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('Dashboard'), findsWidgets);

    final triggerButton = find.text('Build My Website');
    expect(triggerButton, findsOneWidget);
    await tester.tap(triggerButton);
    await tester.pumpAndSettle();

    expect(find.text('Website Builder'), findsOneWidget);
    expect(find.text('Choose a Template'), findsOneWidget);

    await tester.tap(find.text('E-commerce'));
    await tester.pumpAndSettle();
    expect(find.text('Use this template →'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Brand Colors & Logo'), findsOneWidget);
    await tester.tap(find.byType(ElevatedButton).first);
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Add your first product or service'), findsOneWidget);
    await tester.enterText(find.byType(TextField).first, 'Awesome Course');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Connect a domain'), findsOneWidget);
    await tester.tap(find.text('Next'));
    await tester.pumpAndSettle();

    expect(find.text('Ready to Go Live'), findsOneWidget);
    expect(find.text('Publish'), findsOneWidget);

    expect(find.text('Template: E-commerce'), findsOneWidget);
    expect(find.text('Product: Awesome Course'), findsOneWidget);
    expect(find.text('Domain: mybusiness.ohc.app'), findsOneWidget);

    await tester.tap(find.text('Publish'));
    await tester.pumpAndSettle();

    expect(find.text('Dashboard'), findsWidgets);
    expect(find.text('Website Published!'), findsOneWidget);
  });
}