// CUJ: Dashboard UX Simplification
//
// Covers the critical user journey for dashboard text verification:
//   1. Dashboard loads from API
//   2. Verifies plain language UI changes like "Total AI Agents", "System Health", etc.
//   3. No technical jargon is exposed.

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

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

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Dashboard UX Simplification', () {
    late MockHttpClient mockClient;
    late ApiService apiService;

    setUp(() {
      mockClient = MockHttpClient();
      apiService = ApiService(baseUrl: 'http://localhost:8080', token: 'mock-token', client: mockClient);
    });

    testWidgets('Dashboard uses plain language terms', (WidgetTester tester) async {
      tester.view.physicalSize = const Size(1920, 1080);
      tester.view.devicePixelRatio = 1.0;

      final dashboardJson = {
        'organization': {
          'id': 'org_123',
          'name': 'Test Org',
          'domain': 'test.com',
          'members': [],
          'role_profiles': []
        },
        'meetings': [],
        'costs': {
          'total_cost_usd': 0.0,
          'total_tokens': 0,
          'agents': []
        },
        'agents': [
          {'id': '1', 'name': 'Agent1', 'role': 'Support', 'is_running': true, 'created_at': '2025-01-01T00:00:00Z', 'capabilities': []}
        ],
        'statuses': [],
        'updated_at': '2025-01-01T00:00:00Z'
      };

      when(() => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path == '/api/dashboard')),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response(jsonEncode(dashboardJson), 200));

      // mock sub-requests if any
      when(() => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path != '/api/dashboard')),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response('[]', 200));

      await tester.pumpWidget(_wrapScreen(const DashboardScreen(), apiService));

      // using pump instead of pumpAndSettle to avoid infinite animation loop timeouts
      await tester.pump(const Duration(seconds: 1));
      await tester.pump(const Duration(seconds: 1));

      // Verify old jargon is gone
      expect(find.text('Active Pods'), findsNothing);
      expect(find.text('Latency (Avg)'), findsNothing);
      expect(find.text('Active Missions'), findsNothing);
      expect(find.text('Health Score'), findsNothing);

      // Verify plain language texts exist
      expect(find.textContaining('Total AI Agents'), findsOneWidget);
      expect(find.textContaining('Speed (Avg)'), findsOneWidget);
      expect(find.textContaining('Active Tasks'), findsWidgets);
      expect(find.textContaining('System Health'), findsOneWidget);
      expect(find.textContaining('System Overview'), findsOneWidget);
      await tester.drag(find.byType(ListView).first, const Offset(0, -2000)); await tester.pump(const Duration(seconds: 1)); await tester.drag(find.byType(ListView).first, const Offset(0, -2000)); await tester.pump(const Duration(seconds: 1));
      expect(find.textContaining('Manage your AI team.'), findsOneWidget);

      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
    });
  });
}
