import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/shared_task.dart';

void main() {
  group('SharedTask Model', () {
    test('fromJson parses correctly', () {
      final json = {
        'id': '123',
        'title': 'Test Task',
        'assignedAgent': 'Agent X',
        'status': 'COMPLETED',
        'dependencies': ['dep1', 'dep2'],
      };

      final task = SharedTask.fromJson(json);

      expect(task.id, '123');
      expect(task.title, 'Test Task');
      expect(task.assignedAgent, 'Agent X');
      expect(task.status, 'COMPLETED');
      expect(task.dependencies, ['dep1', 'dep2']);
    });

    test('fromJson handles null assignedAgent and empty dependencies', () {
      final json = {
        'id': '124',
        'title': 'Another Task',
        'status': 'PENDING',
      };

      final task = SharedTask.fromJson(json);

      expect(task.id, '124');
      expect(task.title, 'Another Task');
      expect(task.assignedAgent, isNull);
      expect(task.status, 'PENDING');
      expect(task.dependencies, isEmpty);
    });
  });
}
