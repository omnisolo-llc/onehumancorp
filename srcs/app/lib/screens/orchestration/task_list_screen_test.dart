import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/models/task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/services/auth_service.dart';

class MockApiService extends ApiService {
  MockApiService() : super(baseUrl: 'http://test', token: 'test');

  @override
  Future<List<Task>> listTasks() async {
    return [
      const Task(id: '1', title: 'Test Task', status: 'PENDING', assignedAgent: 'Agent1', dependencies: ['Dep1']),
    ];
  }
}

void main() {
  testWidgets('TaskListScreen renders TaskGlassCard and Glassmorphism styling', (WidgetTester tester) async {
    final mockApi = MockApiService();

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApi),
        ],
        child: const MaterialApp(
          home: TaskListScreen(),
        ),
      ),
    );

    await tester.pumpAndSettle();

    expect(find.text('Shared Task List'), findsOneWidget);
    expect(find.text('Test Task'), findsOneWidget);
    expect(find.text('Status: PENDING'), findsOneWidget);
    expect(find.text('Agent: Agent1'), findsOneWidget);
    expect(find.text('Dependencies: Dep1'), findsOneWidget);
    expect(find.byType(TaskGlassCard), findsOneWidget);
  });
}
