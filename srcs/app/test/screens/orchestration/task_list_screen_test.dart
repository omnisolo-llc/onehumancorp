import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/task.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  group('TaskListScreen', () {
    late MockApiService mockApiService;

    setUp(() {
      mockApiService = MockApiService();
    });

    testWidgets('renders loading state initially', (WidgetTester tester) async {
      when(() => mockApiService.listSwarmTasks())
          .thenAnswer((_) async => []);

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            apiServiceProvider.overrideWithValue(mockApiService),
          ],
          child: const MaterialApp(
            home: TaskListScreen(),
          ),
        ),
      );

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('renders list of tasks when data is loaded', (WidgetTester tester) async {
      final tasks = [
        const SwarmTask(id: '1', title: 'Task 1', status: 'COMPLETED'),
        const SwarmTask(id: '2', title: 'Task 2', status: 'IN_PROGRESS', assignedAgentId: 'agent1', dependencies: ['1']),
      ];

      when(() => mockApiService.listSwarmTasks())
          .thenAnswer((_) async => tasks);

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            apiServiceProvider.overrideWithValue(mockApiService),
          ],
          child: const MaterialApp(
            home: TaskListScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('Task 1'), findsOneWidget);
      expect(find.text('Task 2'), findsOneWidget);
      expect(find.text('COMPLETED'), findsOneWidget);
      expect(find.text('IN_PROGRESS'), findsOneWidget);
      expect(find.text('Assigned Agent: Unassigned'), findsOneWidget);
      expect(find.text('Assigned Agent: agent1'), findsOneWidget);
      expect(find.text('Dependencies: 1'), findsOneWidget);
    });

    testWidgets('renders empty state when no tasks are found', (WidgetTester tester) async {
      when(() => mockApiService.listSwarmTasks())
          .thenAnswer((_) async => []);

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            apiServiceProvider.overrideWithValue(mockApiService),
          ],
          child: const MaterialApp(
            home: TaskListScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('No swarm tasks found.'), findsOneWidget);
    });

    testWidgets('renders error state when api throws', (WidgetTester tester) async {
      when(() => mockApiService.listSwarmTasks())
          .thenThrow(Exception('Failed to load'));

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            apiServiceProvider.overrideWithValue(mockApiService),
          ],
          child: const MaterialApp(
            home: TaskListScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.textContaining('Error: Exception: Failed to load'), findsOneWidget);
    });
  });
}
