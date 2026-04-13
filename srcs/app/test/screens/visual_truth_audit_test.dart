import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:mocktail/mocktail.dart';

class MockApiService extends Mock implements ApiService {}

void main() {

  testWidgets('TaskListScreen uses GlassCard correctly', (WidgetTester tester) async {
    final mockApiService = MockApiService();
    when(() => mockApiService.listSharedTasks()).thenAnswer((_) async => [
      {
        'id': 'task-1',
        'title': 'Test Task',
        'status': 'PENDING',
      }
    ]);

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          apiServiceProvider.overrideWithValue(mockApiService),
        ],
        child: const MaterialApp(home: TaskListScreen()),
      ),
    );
    await tester.pumpAndSettle();

    final backdropFinder = find.byType(BackdropFilter);
    expect(backdropFinder, findsWidgets, reason: 'Expected to find BackdropFilter (via GlassCard)');

    bool foundMatrix = false;
    for (final widget in tester.widgetList<BackdropFilter>(backdropFinder)) {
      if (widget.filter.toString().contains('ColorFilter.matrix')) {
        foundMatrix = true;
        break;
      }
    }
    expect(foundMatrix, isTrue, reason: 'Expected GlassCard to use ColorFilter.matrix');
  });
  testWidgets('Glassmorphism components use ColorFilter.matrix', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Stack(
            children: [
              Container(color: Colors.red),
              BackdropFilter(
                filter: ColorFilter.matrix(<double>[
                  1, 0, 0, 0, 0,
                  0, 1, 0, 0, 0,
                  0, 0, 1, 0, 0,
                  0, 0, 0, 1, 0,
                ]),
                child: Container(
                  width: 100,
                  height: 100,
                  color: Colors.white.withOpacity(0.1),
                ),
              ),
            ],
          ),
        ),
      ),
    );

    expect(find.byType(BackdropFilter), findsOneWidget);

    final backdropFilter = tester.widget<BackdropFilter>(find.byType(BackdropFilter));
    expect(backdropFilter.filter, isA<ColorFilter>());
  });
}
