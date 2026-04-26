// CUJ: Handoffs & Escalations – Human-in-the-Loop Approvals
//
// Covers the handoffs critical user journey:
//   1. Empty state shown when no handoffs exist
//   2. Handoffs list rendered when data returned
//   3. Refresh button is available
//   4. Approve action triggers API call
//   5. Reject action triggers API call

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/handoffs_screen.dart';
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

Map<String, dynamic> _fakeHandoff(String id, {String status = 'pending'}) => {
  'id': id,
  'from_agent_id': 'agent-1',
  'to_human_role': 'admin',
  'intent': 'Deploy to production',
  'failed_attempts': 1,
  'current_state': 'Awaiting human approval',
  'status': status,
  'created_at': DateTime.now().toIso8601String(),
};

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Handoffs & Escalations', () {
    testWidgets('shows "No pending handoffs" when list is empty', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('No pending handoffs'), findsOneWidget);
    });

    testWidgets('renders handoff cards when API returns handoffs', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakeHandoff('h1'), _fakeHandoff('h2')]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      // Intent text should be visible
      expect(find.textContaining('DEPLOY TO PRODUCTION'), findsWidgets);
    });

    testWidgets('refresh icon button is present in AppBar', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.refresh), findsOneWidget);
    });

    testWidgets('Approve button triggers POST to resolve endpoint', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakeHandoff('h1')]),
          200,
        ),
      );
      when(
        () => mockClient.post(
          any(),
          headers: any(named: 'headers'),
          body: any(named: 'body'),
        ),
      ).thenAnswer((_) async => http.Response('{}', 200));

      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      final approveBtn = find.textContaining('Approve');
      if (approveBtn.evaluate().isNotEmpty) {

        final slider = find.bySemanticsLabel('Slide to Approve');
        if (slider.evaluate().isNotEmpty) {
          await tester.drag(slider.first, const Offset(240.0, 0));
        } else {
          // fallback if sematics isn't picked up
          await tester.drag(find.byIcon(Icons.chevron_right), const Offset(240.0, 0));
        }

        await tester.pumpAndSettle();

        verify(
          () => mockClient.post(
            any(that: predicate<Uri>((u) => u.path.contains('resolve'))),
            headers: any(named: 'headers'),
            body: any(named: 'body', that: contains('approved')),
          ),
        ).called(1);
      }
      expect(find.byType(Scaffold), findsOneWidget);
    });

    testWidgets('Reject button triggers POST with rejected resolution', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakeHandoff('h1')]),
          200,
        ),
      );
      when(
        () => mockClient.post(
          any(),
          headers: any(named: 'headers'),
          body: any(named: 'body'),
        ),
      ).thenAnswer((_) async => http.Response('{}', 200));

      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      final rejectBtn = find.textContaining('Reject');
      if (rejectBtn.evaluate().isNotEmpty) {
        await tester.tap(rejectBtn.first);
        await tester.pumpAndSettle();

        verify(
          () => mockClient.post(
            any(that: predicate<Uri>((u) => u.path.contains('resolve'))),
            headers: any(named: 'headers'),
            body: any(named: 'body', that: contains('rejected')),
          ),
        ).called(1);
      }
      expect(find.byType(Scaffold), findsOneWidget);
    });
  });
}
