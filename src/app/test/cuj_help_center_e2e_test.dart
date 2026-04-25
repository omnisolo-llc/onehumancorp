// CUJ: Navigate to Help Center
//
// Covers the help center critical user journey:
//   1. Start from the dashboard directly using the AppShell via a customized GoRouter to isolate the UI navigation.
//   2. Navigate to Dashboard (shell)
//   3. Click Help button in AppBar
//   4. Verify Help Center screen is displayed

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/router.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';
import 'package:ohc_app/screens/help_center_screen.dart';
import 'package:ohc_app/services/api_service.dart';

// ── Mocks & Fakes ─────────────────────────────────────────────────────────

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, ApiService api) {
  final router = GoRouter(
    initialLocation: '/dashboard',
    routes: [
      GoRoute(path: '/help', builder: (context, state) => const HelpCenterScreen()),
      ShellRoute(
        builder: (context, state, child) => AppShell(child: child),
        routes: [
          GoRoute(
            path: '/dashboard',
            builder: (context, state) => screen,
          ),
        ],
      ),
    ],
  );
  return ProviderScope(
    overrides: [apiServiceProvider.overrideWithValue(api)],
    child: MaterialApp.router(routerConfig: router),
  );
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Navigate to Help Center', () {
    testWidgets('user can navigate to help center from dashboard', (tester) async {
      final mockClient = MockHttpClient();

      // Mock dashboard endpoints so the dashboard renders successfully
      when(() => mockClient.get(
        any(that: predicate<Uri>((u) => u.path.contains('/api/dashboard/summary'))),
        headers: any(named: 'headers'),
      )).thenAnswer((_) async => http.Response(jsonEncode({
        'active_agents': 2,
        'pending_tasks': 5,
        'open_meetings': 1,
      }), 200));

      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok-ok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const DashboardScreen(), api));
      await tester.pumpAndSettle();

      // We should be on the dashboard.
      expect(find.text('Dashboard'), findsWidgets);

      // Find the Help button in the AppBar actions
      final helpButton = find.byIcon(Icons.help_outline);
      expect(helpButton, findsOneWidget);

      // Tap the Help button
      await tester.tap(helpButton);
      await tester.pumpAndSettle();

      // Verify the Help Center screen is displayed
      expect(find.text('How can we help?'), findsOneWidget);
      expect(find.text('Ask AI'), findsOneWidget);
    });
  });
}
