import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/agent.dart';

void main() {
  group('Agent Model Tests', () {
    test('formattedRole correctly handles known acronyms', () {
      final agent1 = Agent(
        id: '1',
        name: 'test',
        role: 'AI_NEWS_COLLECTOR',
        status: 'pending',
        organizationId: '1',
        createdAt: DateTime.now(),
      );
      expect(agent1.formattedRole, 'AI News Collector');

      final agent2 = Agent(
        id: '2',
        name: 'test',
        role: 'ceo_assistant',
        status: 'pending',
        organizationId: '1',
        createdAt: DateTime.now(),
      );
      expect(agent2.formattedRole, 'CEO Assistant');
    });

    test('isRunning correctly identifies active states', () {
      final agent1 = Agent(id: '1', name: 'test', role: '', status: 'running', organizationId: '1', createdAt: DateTime.now());
      expect(agent1.isRunning, true);

      final agent2 = Agent(id: '2', name: 'test', role: '', status: 'ACTIVE', organizationId: '1', createdAt: DateTime.now());
      expect(agent2.isRunning, true);

      final agent3 = Agent(id: '3', name: 'test', role: '', status: 'pending', organizationId: '1', createdAt: DateTime.now());
      expect(agent3.isRunning, false);
    });

    test('isPending correctly identifies pending states', () {
      final agent1 = Agent(id: '1', name: 'test', role: '', status: 'pending', organizationId: '1', createdAt: DateTime.now());
      expect(agent1.isPending, true);

      final agent2 = Agent(id: '2', name: 'test', role: '', status: 'IDLE', organizationId: '1', createdAt: DateTime.now());
      expect(agent2.isPending, true);

      final agent3 = Agent(id: '3', name: 'test', role: '', status: 'running', organizationId: '1', createdAt: DateTime.now());
      expect(agent3.isPending, false);
    });
  });
}
