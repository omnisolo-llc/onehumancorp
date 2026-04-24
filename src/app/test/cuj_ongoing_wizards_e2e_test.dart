import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/ongoing_management_wizards.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';
import 'package:shared_preferences/shared_preferences.dart';

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

Widget _wrapScreen(Widget screen, {ApiService? api}) {
  final router = GoRouter(
    routes: [
      GoRoute(path: '/', builder: (context, state) => screen),
      GoRoute(path: '/agents', builder: (context, state) => const Scaffold(body: Text('Agents'))),
    ],
  );
  return ProviderScope(
    overrides: [
      if (api != null) apiServiceProvider.overrideWithValue(api),
    ],
    child: MaterialApp.router(routerConfig: router),
  );
}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
    SharedPreferences.setMockInitialValues({'auth_token': 'fake_token_for_test'});
  });

  group('FixThisWizardScreen E2E', () {
    late MockHttpClient mockHttpClient;
    late ApiService apiService;

    setUp(() {
      mockHttpClient = MockHttpClient();
      apiService = ApiService(baseUrl: 'http://localhost:18789', client: mockHttpClient);
    });

    testWidgets('Full flow UI -> DB -> UI works for fix wizard', (tester) async {
      when(() => mockHttpClient.post(any(), headers: any(named: 'headers'), body: any(named: 'body')))
          .thenAnswer((_) async => http.Response(jsonEncode({}), 200));

      await tester.pumpWidget(_wrapScreen(const FixThisWizardScreen(agentId: 'a1'), api: apiService));
      await tester.pumpAndSettle();

      expect(find.text('Help me fix this'), findsOneWidget);
      expect(find.text('View Suggested Fix'), findsOneWidget);

      await tester.tap(find.text('View Suggested Fix'));
      await tester.pumpAndSettle();

      expect(find.text('Apply Fix'), findsOneWidget);

      await tester.tap(find.text('Apply Fix'));
      await tester.pump(); // Start applying
      await tester.pumpAndSettle();

      verify(() => mockHttpClient.post(Uri.parse('http://localhost:18789/api/v1/sync'), headers: any(named: 'headers'))).called(1);

      expect(find.text('Return to Agents'), findsOneWidget);

      await tester.tap(find.text('Return to Agents'));
      await tester.pumpAndSettle();
      expect(find.text('Agents'), findsOneWidget);
    });
  });
}
