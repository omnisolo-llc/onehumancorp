import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/router.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:ohc_app/widgets/welcome_checklist_widget.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

Map<String, dynamic> _fakeDashboard() => {
  'organization': {
    'id': 'org-1',
    'name': 'Test Corp',
    'domain': 'test.com',
    'tier': 'Free',
    'members': [],
  },
  'meetings': [],
  'costs': {
    'totalCostUsd': 123.45,
    'totalTokens': 1000,
    'totalActions': 50,
    'breakdown': {},
  },
  'storage': {
    'usedBytes': 50000000,
    'limitBytes': 500000000,
  },
  'agents': [],
  'statuses': [],
  'updatedAt': DateTime.now().toIso8601String(),
  'business': {
    'name': 'Test Business',
    'description': 'Test Description',
    'stage': 'startup',
    'competitors': [],
  },
  'telemetry': {
    'activeTraces': 0,
    'requestsPerMinute': 0.0,
    'errorRate': 0.0,
    'p99LatencyMs': 0.0,
  },
  'swarmObservability': {
    'totalTokens': 100,
    'promptTokens': 50,
    'completionTokens': 50,
    'costUsd': 0.01,
    'averageLatencyMs': 100,
    'errors': 0,
    'requests': 10,
    'agentMetrics': {},
  },
  'hybridTelemetry': {
    'activeWorkers': 0,
    'queueDepth': 0,
    'processingRate': 0,
    'errorRate': 0,
    'syncLatencyMs': 0,
  },
};

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  testWidgets('Welcome Checklist E2E Flow', (WidgetTester tester) async {
    SharedPreferences.setMockInitialValues({'flutter.auth_token': 'mock_token'});

    final mockClient = MockHttpClient();
    when(() => mockClient.get(any(), headers: any(named: 'headers')))
        .thenAnswer((_) async => http.Response(jsonEncode(_fakeDashboard()), 200));

    final api = ApiService(baseUrl: 'http://localhost', token: 'mock_token', client: mockClient);
    final authService = AuthService(baseUrl: 'http://localhost', client: mockClient);

    final router = GoRouter(
      initialLocation: '/',
      routes: [
        GoRoute(
          path: '/',
          builder: (context, state) => const DashboardScreen(),
        ),
      ],
    );

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          authServiceProvider.overrideWithValue(authService),
          apiServiceProvider.overrideWithValue(api),
        ],
        child: MaterialApp.router(routerConfig: router),
      ),
    );

    // Initial pump and settle
    await tester.pumpAndSettle();
    await tester.pump(const Duration(seconds: 1));
    await tester.pumpAndSettle();

    // Verify Welcome Checklist is visible on the dashboard
    await tester.scrollUntilVisible(find.byType(WelcomeChecklistWidget), 50.0, scrollable: find.byType(Scrollable).first);
    expect(find.byType(WelcomeChecklistWidget), findsOneWidget);
    expect(find.text('Welcome Checklist'), findsOneWidget);
    expect(find.text('Business live'), findsOneWidget);
    expect(find.text('Add 3 more products'), findsOneWidget);
    expect(find.text('Connect Instagram'), findsOneWidget);
    expect(find.text('Share your link with a friend'), findsOneWidget);
  });
}
