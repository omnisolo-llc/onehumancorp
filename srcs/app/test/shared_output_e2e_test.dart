import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/screens/shared_output_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';
import 'package:ohc_app/models/shared_task.dart';

class MockApiService extends Mock implements ApiService {}

void main() {
  late MockApiService mockApiService;

  setUp(() {
    mockApiService = MockApiService();
    registerFallbackValue(Uri());
  });

  testWidgets('Full Viral Loop: Share → View', (WidgetTester tester) async {
    // Increase screen size to avoid hit test errors
    tester.view.physicalSize = const Size(1920, 1080);
    tester.view.devicePixelRatio = 1.0;

    final mockTask = {
      'id': 'task-1',
      'title': 'Completed Task',
      'status': 'COMPLETED',
      'agent_id': 'agent-1',
    };

    final mockShared = {
      'id': 'shared-1',
      'token': 'secret-token',
      'taskId': 'task-1',
      'content': 'Result content',
      'author': 'agent-1',
    };

    when(() => mockApiService.listSharedTasks()).thenAnswer((_) async => [mockTask]);
    when(() => mockApiService.shareOutput(
          taskId: any(named: 'taskId'),
          content: any(named: 'content'),
          author: any(named: 'author'),
        )).thenAnswer((_) async => mockShared);
    when(() => mockApiService.getSharedOutput('secret-token')).thenAnswer((_) async => mockShared);

    // 1. Share Flow
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
    expect(find.text('Share Result'), findsOneWidget);

    await tester.tap(find.text('Share Result'));
    await tester.pumpAndSettle();

    verify(() => mockApiService.shareOutput(
      taskId: 'task-1',
      content: any(named: 'content'),
      author: 'agent-1',
    )).called(1);

    // 2. View Flow
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(
          home: SharedOutputScreen(token: 'secret-token'),
        ),
      ),
    );

    await tester.pump(); // Future start
    await tester.pump(); // Finish

    expect(find.text('Agentic Intelligence Shared'), findsOneWidget);
    expect(find.text('Author: agent-1'), findsOneWidget);
    expect(find.text('Result content'), findsOneWidget);

    // Reset view
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
  });
}
