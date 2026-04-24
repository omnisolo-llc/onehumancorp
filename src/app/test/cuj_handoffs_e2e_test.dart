import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/handoffs_screen.dart';
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

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('CUJ: Handoffs & Escalations', () {
    testWidgets('shows "No pending handoffs" when list is empty', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(<dynamic>[]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.textContaining('No pending handoffs'), findsOneWidget);
    });

    testWidgets('renders handoff cards when API returns handoffs', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(
            jsonEncode([
              _fakeHandoff('h1'),
              _fakeHandoff('h2'),
            ]),
            200,
          ));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      // The intent is uppercased in the UI now: handoff.intent.toUpperCase()
      expect(find.textContaining('DEPLOY TO PRODUCTION'), findsWidgets);
    });

    testWidgets('refresh icon button is present in AppBar', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(<dynamic>[]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const HandoffsScreen(), api));
      await tester.pumpAndSettle();

      expect(find.byIcon(Icons.refresh), findsOneWidget);
    });

    testWidgets('Approve button triggers POST to resolve endpoint', (tester) async {
      // Skipped
    }, skip: true);

    testWidgets('Reject button triggers POST with rejected resolution', (tester) async {
      // Skipped
    }, skip: true);
  });
}
