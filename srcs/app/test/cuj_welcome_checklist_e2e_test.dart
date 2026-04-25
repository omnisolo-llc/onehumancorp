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

Widget _wrapScreen(Widget screen, ApiService api, {String initialRoute = '/'}) {
  final router = GoRouter(
    initialLocation: initialRoute,
    routes: [
      GoRoute(path: '/', builder: (context, state) => const Scaffold(body: Text('Home Dummy'))),
      GoRoute(path: '/dashboard', builder: (context, state) => screen),
      GoRoute(path: '/service', builder: (context, state) => const Scaffold(body: Text('Service Dummy'))),
      GoRoute(path: '/integrations', builder: (context, state) => const Scaffold(body: Text('Integrations Dummy'))),
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

  group('CUJ: Welcome Checklist Post-Onboarding', () {
    late MockHttpClient mockClient;
    late ApiService apiService;

    setUp(() {
      mockClient = MockHttpClient();
      apiService = ApiService(baseUrl: 'http://localhost', token: 'mock_token', client: mockClient);
    });

    testWidgets('Dashboard Welcome Checklist navigates to Service correctly', (tester) async {
      tester.view.physicalSize = const Size(1920, 3000); // Taller view
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
        'agents': [],
        'statuses': [],
        'updated_at': '2025-01-01T00:00:00Z'
      };

      when(() => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path == '/api/dashboard')),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response(jsonEncode(dashboardJson), 200));

      when(() => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path != '/api/dashboard')),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response('[]', 200));

      await tester.pumpWidget(_wrapScreen(const DashboardScreen(), apiService, initialRoute: '/dashboard'));
      await tester.pump(const Duration(seconds: 1));

      expect(find.byType(ListView), findsWidgets);

      // Ensure the widget is visible
      final target = find.text("Add 3 more products").first;
      await tester.scrollUntilVisible(target, 50.0, scrollable: find.byType(Scrollable).first);
      await tester.pump(const Duration(seconds: 1));

      // Tap navigation
      await tester.tap(target);
      await tester.pump(const Duration(seconds: 1));
      await tester.pump(const Duration(seconds: 1));

      expect(find.text("Service Dummy"), findsOneWidget);

      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
    });

    testWidgets('Dashboard Welcome Checklist navigates to Integrations correctly', (tester) async {
      tester.view.physicalSize = const Size(1920, 3000); // Taller view
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
        'agents': [],
        'statuses': [],
        'updated_at': '2025-01-01T00:00:00Z'
      };

      when(() => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path == '/api/dashboard')),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response(jsonEncode(dashboardJson), 200));

      when(() => mockClient.get(
        any(that: predicate<Uri>((uri) => uri.path != '/api/dashboard')),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response('[]', 200));

      await tester.pumpWidget(_wrapScreen(const DashboardScreen(), apiService, initialRoute: '/dashboard'));
      await tester.pump(const Duration(seconds: 1));

      expect(find.byType(ListView), findsWidgets);

      final target = find.text("Connect Instagram").first;
      await tester.scrollUntilVisible(target, 50.0, scrollable: find.byType(Scrollable).first);
      await tester.pump(const Duration(seconds: 1));

      // Tap navigation
      await tester.tap(target);
      await tester.pump(const Duration(seconds: 1));
      await tester.pump(const Duration(seconds: 1));

      expect(find.text("Integrations Dummy"), findsOneWidget);

      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
    });
  });
}
