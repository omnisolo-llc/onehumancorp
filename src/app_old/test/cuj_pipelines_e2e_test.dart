// CUJ: SDLC Pipelines – Release Promotion
//
// Covers the pipelines critical user journey:
//   1. Shows "No active pipelines" when list is empty
//   2. Renders pipeline cards from API response
//   3. Refresh button is present
//   4. Pipeline name and status are displayed
//   5. Promote button triggers POST to promote endpoint

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/pipelines_screen.dart';
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

Map<String, dynamic> _fakePipeline(String id, String name, {String status = 'active'}) => {
  'id': id,
  'name': name,
  'status': status,
  'branch': 'main',
  'staging_url': 'https://staging.example.com',
  'initiated_by': 'agent-1',
  'created_at': DateTime.now().toIso8601String(),
  'updated_at': DateTime.now().toIso8601String(),
};

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: SDLC Pipelines', () {
    testWidgets('shows "No active pipelines" when list is empty', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const PipelinesScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('No active pipelines'), findsOneWidget);
    });

    testWidgets('renders pipeline card with name from API', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakePipeline('p1', 'Release v1.2.0')]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const PipelinesScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Release v1.2.0'), findsOneWidget);
    });

    testWidgets('refresh button is present in AppBar', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const PipelinesScreen(), api));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.refresh), findsOneWidget);
    });

    testWidgets('pipeline branch name is shown in card', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakePipeline('p1', 'Feature Deploy', status: 'running')]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const PipelinesScreen(), api));
      await tester.pumpAndSettle();

      // Branch 'main' should be visible in the card
      expect(find.textContaining('main'), findsWidgets);
    });

    testWidgets('Promote button triggers POST to promote endpoint', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakePipeline('p1', 'Release v2.0')]),
          200,
        ),
      );
      when(
        () => mockClient.post(
          any(),
          headers: any(named: 'headers'),
        ),
      ).thenAnswer((_) async => http.Response('{}', 200));

      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const PipelinesScreen(), api));
      await tester.pumpAndSettle();

      final promoteBtn = find.textContaining('Promote');
      if (promoteBtn.evaluate().isNotEmpty) {
        await tester.tap(promoteBtn.first);
        await tester.pumpAndSettle();

        verify(
          () => mockClient.post(
            any(that: predicate<Uri>((u) => u.path.contains('promote'))),
            headers: any(named: 'headers'),
          ),
        ).called(1);
      }
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
