import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/task_model.dart';

void main() {
  group('Task Model', () {
    test('fromJson successfully parses valid JSON', () {
      final json = {
        'id': 'task-123',
        'title': 'Test the shared list',
        'status': 'IN_PROGRESS',
        'assignedAgent': 'Echo',
        'dependencies': ['dep1', 'dep2'],
      };

      final task = Task.fromJson(json);

      expect(task.id, 'task-123');
      expect(task.title, 'Test the shared list');
      expect(task.status, 'IN_PROGRESS');
      expect(task.assignedAgent, 'Echo');
      expect(task.dependencies, ['dep1', 'dep2']);
    });

    test('fromJson handles missing values with defaults', () {
      final json = <String, dynamic>{};

      final task = Task.fromJson(json);

      expect(task.id, '');
      expect(task.title, '');
      expect(task.status, 'PENDING');
      expect(task.assignedAgent, 'Unassigned');
      expect(task.dependencies, isEmpty);
    });
  });
}
