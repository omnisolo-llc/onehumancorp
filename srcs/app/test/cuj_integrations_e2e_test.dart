import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/integrations_screen.dart';
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

  group('CUJ: Integrations & MCP Tools', () {
    testWidgets('screen renders External Channels section', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(<dynamic>[]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle(const Duration(milliseconds: 100)); await tester.pump(const Duration(seconds: 1));

      expect(find.text('External Channels'), findsOneWidget);
    });

    testWidgets('Telegram integration card is displayed', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(<dynamic>[]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle(const Duration(milliseconds: 100)); await tester.pump(const Duration(seconds: 1));

      expect(find.text('Telegram'), findsOneWidget);
    });

    testWidgets('Discord integration card is displayed', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(<dynamic>[]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle(const Duration(milliseconds: 100)); await tester.pump(const Duration(seconds: 1));

      expect(find.text('Discord'), findsOneWidget);
    });

    testWidgets('Connect button opens dialog when tapped', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(<dynamic>[]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle(const Duration(milliseconds: 100)); await tester.pump(const Duration(seconds: 1));

      await tester.tap(find.text('Configure').first);
      await tester.pumpAndSettle(const Duration(milliseconds: 100)); await tester.pump(const Duration(seconds: 1));

      expect(find.byType(AlertDialog), findsOneWidget);
    });

    testWidgets('MCP tools section renders when tools are available', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(
            jsonEncode([
              {'name': 'TestTool', 'description': 'A dummy tool for testing'}
            ]),
            200,
          ));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);
      await tester.pumpWidget(_wrapScreen(const IntegrationsScreen(), api));
      await tester.pumpAndSettle(const Duration(milliseconds: 100)); await tester.pump(const Duration(seconds: 1));

      expect(find.text('TestTool'), findsOneWidget);
    });
  });
}
