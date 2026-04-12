import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/shared_task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
  });

  Widget createTestableWidget(List<SharedTask>? tasks, {bool isError = false}) {
    return ProviderScope(
      overrides: [
        apiServiceProvider.overrideWithValue(mockApiService),
        sharedTasksProvider.overrideWith((ref) async {
          if (isError) throw Exception('API Error');
          return tasks ?? [];
        }),
      ],
      child: const MaterialApp(
        home: TaskListScreen(),
      ),
    );
  }

  testWidgets('renders error state when API fails', (WidgetTester tester) async {
    await tester.pumpWidget(createTestableWidget(null, isError: true));
    await tester.pumpAndSettle();
    expect(find.textContaining('Error: Exception: API Error'), findsOneWidget);
  });

  testWidgets('renders empty state when no tasks are returned', (WidgetTester tester) async {
    await tester.pumpWidget(createTestableWidget([]));
    await tester.pumpAndSettle();
    expect(find.text('No tasks available'), findsOneWidget);
  });

  testWidgets('renders tasks correctly', (WidgetTester tester) async {
    final tasks = [
      SharedTask(id: 't1', title: 'Task 1', assignedAgent: 'Agent A', status: 'PENDING', dependencies: []),
      SharedTask(id: 't2', title: 'Task 2', assignedAgent: null, status: 'IN_PROGRESS', dependencies: ['t1']),
    ];
    await tester.pumpWidget(createTestableWidget(tasks));
    await tester.pumpAndSettle();

    expect(find.byType(TaskGlassCard), findsNWidgets(2));
    expect(find.text('Task 1'), findsOneWidget);
    expect(find.text('Task 2'), findsOneWidget);
    expect(find.text('Agent A'), findsOneWidget);
    expect(find.text('Unassigned'), findsOneWidget);
    expect(find.text('PENDING'), findsOneWidget);
    expect(find.text('IN_PROGRESS'), findsOneWidget);
    expect(find.text('Dependencies: t1'), findsOneWidget);
  });
}
