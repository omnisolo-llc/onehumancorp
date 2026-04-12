import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/task.dart';

void main() {
  group('SwarmTask Model', () {
    test('fromJson successfully parses valid JSON', () {
      final json = {
        'id': 'task_1',
        'title': 'Test task',
        'status': 'PENDING',
        'assigned_agent_id': 'agent_1',
        'dependencies': ['task_0'],
      };

      final task = SwarmTask.fromJson(json);

      expect(task.id, 'task_1');
      expect(task.title, 'Test task');
      expect(task.status, 'PENDING');
      expect(task.assignedAgentId, 'agent_1');
      expect(task.dependencies, ['task_0']);
    });

    test('fromJson handles missing optional fields', () {
      final json = {
        'id': 'task_1',
        'title': 'Test task',
        'status': 'COMPLETED',
      };

      final task = SwarmTask.fromJson(json);

      expect(task.id, 'task_1');
      expect(task.title, 'Test task');
      expect(task.status, 'COMPLETED');
      expect(task.assignedAgentId, isNull);
      expect(task.dependencies, isEmpty);
    });

    test('fromJson provides defaults for missing required fields', () {
      final json = <String, dynamic>{};

      final task = SwarmTask.fromJson(json);

      expect(task.id, '');
      expect(task.title, '');
      expect(task.status, 'PENDING');
      expect(task.assignedAgentId, isNull);
      expect(task.dependencies, isEmpty);
    });
  });
}
