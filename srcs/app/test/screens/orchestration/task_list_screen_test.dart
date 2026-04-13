import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/models/shared_task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  Widget createTestWidget() {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApiService),
      ],
      child: const MaterialApp(
        home: TaskListScreen(),
      ),
    );
  }

  testWidgets('TaskListScreen shows tasks', (WidgetTester tester) async {
    final tasks = [
      SharedTask(
        id: '1',
        organizationId: 'org-1',
        parentPlanId: '',
        dependencies: [],
        title: 'Test Task 1',
        description: 'Description 1',
        assignedAgentId: 'agent-1',
        status: 'IN_PROGRESS',
        priority: 'P1',
        payload: '{}',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      ),
    ];

    when(() => mockApiService.listAllTasks()).thenAnswer((_) async => tasks);

    await tester.pumpWidget(createTestWidget());
    await tester.pump(); // Start loading
    await tester.pump(); // Finish loading

    expect(find.text('KAIROS Orchestration'), findsOneWidget);
    expect(find.text('Test Task 1'), findsOneWidget);
    expect(find.text('Description 1'), findsOneWidget);
    expect(find.text('IN_PROGRESS'), findsOneWidget);
    expect(find.text('P1'), findsOneWidget);
  });

  testWidgets('TaskListScreen shows empty state', (WidgetTester tester) async {
    when(() => mockApiService.listAllTasks()).thenAnswer((_) async => []);

    await tester.pumpWidget(createTestWidget());
    await tester.pump();
    await tester.pump();

    expect(find.text('No tasks found in the swarm.'), findsOneWidget);
  });
}
