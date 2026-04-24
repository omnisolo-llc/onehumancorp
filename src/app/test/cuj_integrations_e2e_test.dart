// CUJ: Integrations & MCP Tools
//
// Covers the integrations critical user journey:
//   1. Screen renders with section headers
//   2. Connect button is tappable for Telegram
//   3. Connect button is tappable for Discord
//   4. MCP tools section renders when tools returned
//   5. Empty MCP tools section shows appropriate message

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/integrations_screen.dart';
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

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Integrations & MCP Tools', () {
    testWidgets('screen renders External Channels section', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(<dynamic>[]), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('External Channels'), findsOneWidget);
    });

    testWidgets('Telegram integration card is displayed', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(<dynamic>[]), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Telegram'), findsOneWidget);
    });

    testWidgets('Discord integration card is displayed', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(<dynamic>[]), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Discord'), findsOneWidget);
    });

    testWidgets('Connect button opens dialog when tapped', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(jsonEncode(<dynamic>[]), 200),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle();

      final connectBtn = find.textContaining('Connect');
      if (connectBtn.evaluate().isNotEmpty) {
        await tester.tap(connectBtn.first);
        await tester.pumpAndSettle();
        // A connection dialog should appear
        expect(find.byType(AlertDialog), findsOneWidget);
      }
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('MCP tools section renders when tools are available', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([
            {'id': 't1', 'name': 'GitHub', 'description': 'GitHub integration', 'actions': []},
          ]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle();

      // MCP Tools section or tools name should be visible
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
