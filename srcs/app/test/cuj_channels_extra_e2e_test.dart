// CUJ: Chat Channels – Connect Communication Backends
//
// Covers the channels critical user journey:
//   1. Screen renders Chat Channels title
//   2. Add Channel button is present
//   3. Empty state shows "No channels" callout
//   4. Existing channels are listed
//   5. Add Channel dialog opens on button tap

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/channels_screen.dart';
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

Map<String, dynamic> _fakeChannel(String id, String name, String backend) => {
  'id': id,
  'name': name,
  'backend': backend,
  'config': <String, dynamic>{},
  'enabled': true,
  'organization_id': 'org-1',
};

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Chat Channels', () {
    testWidgets('renders Chat Channels title in AppBar', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const ChannelsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Chat Channels'), findsOneWidget);
    });

    testWidgets('Add Channel button is present in AppBar', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const ChannelsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Add Channel').first, findsOneWidget);
    });

    testWidgets('Add Channel opens dialog when tapped', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const ChannelsScreen(), api));
      await tester.pumpAndSettle();

      await tester.tap(find.textContaining('Add Channel').first);
      await tester.pumpAndSettle();

      // A configuration dialog should appear
      expect(find.byType(Dialog), findsOneWidget);
    });

    testWidgets('channel name is shown in list when returned from API', (tester) async {
      final mockClient = MockHttpClient();
      when(
        () => mockClient.get(any(), headers: any(named: 'headers')),
      ).thenAnswer(
        (_) async => http.Response(
          jsonEncode([_fakeChannel('c1', 'Slack Production', 'slack')]),
          200,
        ),
      );
      final api = ApiService(
        baseUrl: 'http://localhost',
        token: 'tok',
        client: mockClient,
      );

      await tester.pumpWidget(_wrapScreen(const ChannelsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Slack Production'), findsOneWidget);
    });

    testWidgets('shows error message when API fails', (tester) async {
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

      await tester.pumpWidget(_wrapScreen(const ChannelsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('Error'), findsWidgets);
    });
  });
}
