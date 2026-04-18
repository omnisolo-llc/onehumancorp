import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/shared_output_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  testWidgets('SharedOutputScreen displays intelligence output', (WidgetTester tester) async {
    final mockData = {
      'id': 'shared-1',
      'token': 'test-token',
      'taskId': 'task-123',
      'content': 'Test intelligence content',
      'author': 'Agent Nova',
      'createdAt': DateTime.now().toIso8601String(),
    };

    when(() => mockApiService.getSharedOutput(any())).thenAnswer((_) async => mockData);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(
          home: SharedOutputScreen(token: 'test-token'),
        ),
      ),
    );

    await tester.pump(); // Start future
    await tester.pump(); // Completion

    expect(find.text('Agentic Intelligence Shared'), findsOneWidget);
    expect(find.text('Author: Agent Nova'), findsOneWidget);
    expect(find.text('Test intelligence content'), findsOneWidget);
    expect(find.text('Join the OHC Swarm'), findsOneWidget);
  });
}
