import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/models/task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  Widget createWidgetUnderTest() {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApiService),
      ],
      child: const MaterialApp(
        home: TaskListScreen(),
      ),
    );
  }

  testWidgets('TaskListScreen renders loading and then empty state', (tester) async {
    when(() => mockApiService.listOrchestrationTasks())
        .thenAnswer((_) async => []);

    await tester.pumpWidget(createWidgetUnderTest());

    // Initially loading
    expect(find.byType(CircularProgressIndicator), findsOneWidget);

    await tester.pumpAndSettle();

    // Then empty state
    expect(find.text('No orchestration tasks available.'), findsOneWidget);
  });

  testWidgets('TaskListScreen renders tasks', (tester) async {
    when(() => mockApiService.listOrchestrationTasks())
        .thenAnswer((_) async => [
              Task(
                id: '1',
                title: 'Test Task 1',
                status: 'COMPLETED',
                dependencies: [],
              ),
              Task(
                id: '2',
                title: 'Test Task 2',
                status: 'IN_PROGRESS',
                assignedAgent: 'Agent Smith',
                dependencies: ['Task 1'],
              ),
            ]);

    await tester.pumpWidget(createWidgetUnderTest());
    await tester.pumpAndSettle();

    expect(find.text('Test Task 1'), findsOneWidget);
    expect(find.text('COMPLETED'), findsOneWidget);
    expect(find.text('Test Task 2'), findsOneWidget);
    expect(find.text('IN_PROGRESS'), findsOneWidget);
    expect(find.text('Assigned Agent: Agent Smith'), findsOneWidget);
    expect(find.text('Dependencies: Task 1'), findsOneWidget);
  });
}
