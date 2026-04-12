import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/services/api_service.dart';
import 'dart:convert';

class MockHttpClient extends Mock implements http.Client {}

class FakeUri extends Fake implements Uri {}

void main() {
  late MockHttpClient mockClient;
  late ApiService apiService;

  setUpAll(() {
    registerFallbackValue(FakeUri());
  });

  setUp(() {
    mockClient = MockHttpClient();
    apiService = ApiService(
      baseUrl: 'http://test.local',
      token: 'fake_token',
      client: mockClient,
    );
  });

  group('getSharedTasks', () {
    test('returns mocked fallback list on 404', () async {
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response('Not Found', 404));

      final tasks = await apiService.getSharedTasks();

      expect(tasks.length, 4);
      expect(tasks[0].id, 't1');
      expect(tasks[0].title, 'Data Ingestion Sync');
    });

    test('returns parsed list on 200', () async {
      final jsonResponse = [
        {
          'id': 'api_1',
          'title': 'API Task',
          'status': 'REVIEW',
          'dependencies': []
        }
      ];

      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenAnswer((_) async => http.Response(jsonEncode(jsonResponse), 200));

      final tasks = await apiService.getSharedTasks();

      expect(tasks.length, 1);
      expect(tasks[0].id, 'api_1');
      expect(tasks[0].status, 'REVIEW');
    });

    test('returns mocked fallback list on connection exception', () async {
      when(() => mockClient.get(any(), headers: any(named: 'headers')))
          .thenThrow(Exception('Connection refused'));

      final tasks = await apiService.getSharedTasks();

      expect(tasks.length, 4);
      expect(tasks[0].id, 't1');
    });
  });
}
