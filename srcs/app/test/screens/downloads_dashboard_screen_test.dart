import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/screens/downloads_dashboard_screen.dart';
import 'package:ohc_app/services/api_service.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  Widget buildTestWidget() {
    return ProviderScope(
      overrides: [apiServiceProvider.overrideWithValue(mockApiService)],
      child: const MaterialApp(home: DownloadsDashboardScreen()),
    );
  }

  testWidgets('displays list of downloads', (tester) async {
    when(() => mockApiService.listDownloads()).thenAnswer(
      (_) async => [
        {
          'id': 'dl-1',
          'os': 'macOS',
          'version': '1.0.0',
          'createdAt': '2026-04-05T12:00:00Z',
        },
      ],
    );

    await tester.pumpWidget(buildTestWidget());
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle();

    expect(find.text('Standalone App Downloads'), findsOneWidget);
    expect(find.text('OS: macOS'), findsOneWidget);
    expect(find.text('Version: 1.0.0'), findsOneWidget);
  });

  testWidgets('displays empty state', (tester) async {
    when(() => mockApiService.listDownloads()).thenAnswer((_) async => []);

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    expect(find.text('No downloads tracked yet.'), findsOneWidget);
  });

  testWidgets('displays error state', (tester) async {
    when(
      () => mockApiService.listDownloads(),
    ).thenAnswer((_) => Future.error(Exception('API failure')));

    await tester.pumpWidget(buildTestWidget());
    await tester.pumpAndSettle();

    expect(find.textContaining('API failure'), findsOneWidget);
  });
}
