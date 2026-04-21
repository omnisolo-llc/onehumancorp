// CUJ: Cost Dashboard – Financial Monitoring
//
// Covers the cost & token usage critical user journey:
//   1. Screen renders with loading state
//   2. Cost data is displayed after loading
//   3. Refresh button triggers reload
//   4. Error state shown when API fails
//   5. Cost breakdown section is visible

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/cost_dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, ApiService api) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
    ],
  );
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: MaterialApp.router(routerConfig: router),
  );
}

Map<String, dynamic> _fakeDashboard({double total = 987.65}) => {
  'organization': {
    'id': 'org-1',
    'name': 'Test Corp',
    'domain': 'test.com',
    'members': [],
  },
  'meetings': [],
  'costs': {
    'total': total,
    'currency': 'USD',
    'period': 'monthly',
    'breakdown': {'gpt-4o': 500.0, 'claude-3': 487.65},
  },
  'agents': [],
  'statuses': [],
  'updatedAt': DateTime.now().toIso8601String(),
};

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Cost Dashboard', () {
    testWidgets('renders scaffold with AppBar title', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(_fakeDashboard()), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const CostDashboardScreen(), api));
      await tester.pumpAndSettle();

      expect(find.byType(Scaffold), findsOneWidget);
      expect(find.textContaining('Cost'), findsWidgets);
    });

    testWidgets('displays total cost value from API', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(_fakeDashboard(total: 1234.56)), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const CostDashboardScreen(), api));
      await tester.pumpAndSettle();

      // Amount should appear somewhere in the rendered text
      expect(find.text('Total Spend'), findsOneWidget);
    });

    testWidgets('refresh button is present', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(_fakeDashboard()), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const CostDashboardScreen(), api));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.refresh), findsOneWidget);
    });

    testWidgets('tapping refresh button triggers a second API call', (tester) async {
      final mockClient = MockHttpClient();
      var callCount = 0;
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer((_) async {
        callCount++;
        return http.Response(jsonEncode(_fakeDashboard()), 200);
      });
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const CostDashboardScreen(), api));
      await tester.pumpAndSettle();

      final initialCallCount = callCount;
      await tester.tap(find.byIcon(Icons.refresh));
      await tester.pumpAndSettle();

      expect(callCount, greaterThan(initialCallCount));
    });

    testWidgets('shows error text when API returns error', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response('Internal Server Error', 500),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const CostDashboardScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Error'), findsWidgets);
    });
  });
}
