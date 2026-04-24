import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/whats_new_screen.dart';
import 'package:ohc_app/screens/api_docs_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockHttpClient extends Mock implements http.Client {}
class FakeUri extends Fake implements Uri {}

void main() {
  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  group('WhatsNewScreen', () {
    testWidgets('renders release notes', (tester) async {
      final mockClient = MockHttpClient();
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode([
            {
              'version': 'v2.4.0',
              'date': '2026-04-24',
              'changes': ['Feature A', 'Feature B']
            }
          ]), 200));

      final api = ApiService(baseUrl: 'http://localhost', token: 'tok', client: mockClient);

      await tester.pumpWidget(ProviderScope(
        overrides: [apiServiceProvider.overrideWithValue(api)],
        child: const MaterialApp(home: WhatsNewScreen()),
      ));
      await tester.pumpAndSettle();

      expect(find.text('v2.4.0'), findsOneWidget);
      expect(find.text('Feature A'), findsOneWidget);
      expect(find.text('2026-04-24'), findsOneWidget);
    });
  });

  group('ApiDocsScreen', () {
    testWidgets('renders API endpoints', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: ApiDocsScreen()));
      expect(find.text('Advanced: OHC Developer API'), findsOneWidget);
      expect(find.text('/api/dashboard'), findsOneWidget);
      expect(find.text('GET'), findsWidgets);
    });
  });
}
