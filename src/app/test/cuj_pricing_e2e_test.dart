import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/cost_dashboard_screen.dart';
import 'package:ohc_app/screens/pricing_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, ApiService api) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
      GoRoute(path: '/pricing', builder: (context, state) => const PricingScreen()),
    ],
  );
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: MaterialApp.router(routerConfig: router),
  );
}

Map<String, dynamic> _fakeDashboard() => {
  'organization': {
    'id': 'org-1',
    'name': 'Test Corp',
    'domain': 'test.com',
    'members': [],
  },
  'meetings': [],
  'costs': {
    'totalCostUSD': 987.65,
    'totalTokens': 150000,
    'currentPlan': 'Starter',
    'aiActionsUsed': 450,
    'aiActionsLimit': 1000,
    'storageUsedGB': 2.5,
    'agents': [],
  },
  'agents': [],
  'statuses': [],
  'updatedAt': DateTime.now().toIso8601String(),
};

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Cost Dashboard & Pricing Page Navigation', () {
    testWidgets('renders my plan section and navigates to pricing', (tester) async {
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);

      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(_fakeDashboard()), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);

      await tester.pumpWidget(_wrapScreen(const CostDashboardScreen(), api));
      await tester.pumpAndSettle();

      // Verify My Plan UI section elements
      expect(find.text('My Plan'), findsOneWidget);
      expect(find.text('Current Plan: Starter'), findsOneWidget);
      expect(find.text('450 / 1000'), findsOneWidget);
      expect(find.textContaining('2.50 GB / 5.0 GB'), findsOneWidget);

      // Upgrade button navigates to pricing
      await tester.drag(find.byType(ListView), const Offset(0, -500));
      await tester.pumpAndSettle();

      final upgradeButton = find.widgetWithText(FilledButton, 'Upgrade');
      if (upgradeButton.evaluate().isNotEmpty) {
          await tester.tap(upgradeButton);
      } else {
          // Try find by just text if it's rendered differently inside GlassCard/FilledButton
          await tester.tap(find.text('Upgrade'));
      }
      await tester.pumpAndSettle();

      expect(find.byType(PricingScreen), findsOneWidget);
      expect(find.text('Free'), findsWidgets);
      expect(find.text('Starter'), findsWidgets);
      expect(find.text('Pro'), findsWidgets);
      expect(find.text('Business'), findsWidgets);
    });
  });
}
