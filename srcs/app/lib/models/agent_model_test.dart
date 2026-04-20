import 'package:flutter_test/flutter_test.dart';
import 'package:ohc_app/models/agent.dart';

void main() {
  group('Agent model', () {
    test('fromJson parses all fields', () {
      final json = {
        'id': 'a1',
        'name': 'Alice',
        'role': 'engineer',
        'status': 'running',
        'organization_id': 'org-1',
        'created_at': '2025-01-01T00:00:00Z',
        'svid_verified': true,
      };
      final agent = Agent.fromJson(json);
      expect(agent.id, 'a1');
      expect(agent.name, 'Alice');
      expect(agent.role, 'engineer');
      expect(agent.status, 'running');
      expect(agent.organizationId, 'org-1');
      expect(agent.svidVerified, isTrue);
      expect(agent.isRunning, isTrue);
      expect(agent.isPending, isFalse);
    });

    test('fromJson uses defaults for missing optional fields', () {
      final json = {'id': 'b2', 'name': 'Bob'};
      final agent = Agent.fromJson(json);
      expect(agent.role, '');
      expect(agent.status, 'pending');
      expect(agent.organizationId, '');
      expect(agent.svidVerified, isFalse);
      expect(agent.isPending, isTrue);
    });

    test('toJson round-trips', () {
      final json = {
        'id': 'c3',
        'name': 'Carol',
        'role': 'ceo',
        'status': 'pending',
        'organization_id': 'org-2',
        'created_at': '2025-06-01T12:00:00.000Z',
        'svid_verified': true,
      };
      final agent = Agent.fromJson(json);
      final out = agent.toJson();
      expect(out['id'], 'c3');
      expect(out['name'], 'Carol');
      expect(out['role'], 'ceo');
      expect(out['svid_verified'], true);
    });

    test('formattedRole formats roles correctly', () {
      final agent1 = Agent(
        id: '1', name: 'N1', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'AI_NEWS_COLLECTOR',
      );
      expect(agent1.formattedRole, 'AI News Collector');

      final agent2 = Agent(
        id: '2', name: 'N2', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'ceo',
      );
      expect(agent2.formattedRole, 'CEO');

      final agent3 = Agent(
        id: '3', name: 'N3', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'QA_engineer',
      );
      expect(agent3.formattedRole, 'QA Engineer');

      final agent4 = Agent(
        id: '4', name: 'N4', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'cfo_assistant',
      );
      expect(agent4.formattedRole, 'CFO Assistant');

      final agent5 = Agent(
        id: '5', name: 'N5', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'seo_expert',
      );
      expect(agent5.formattedRole, 'SEO Expert');

      final agent6 = Agent(
        id: '6', name: 'N6', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'llm_researcher',
      );
      expect(agent6.formattedRole, 'LLM Researcher');

      final agent7 = Agent(
        id: '7', name: 'N7', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: '',
      );
      expect(agent7.formattedRole, '');

      final agent8 = Agent(
        id: '8', name: 'N8', status: 'running', organizationId: 'o1', createdAt: DateTime.now(),
        role: 'software__engineer',
      );
      expect(agent8.formattedRole, 'Software  Engineer');
    });
  });

  group('AgentProvider model', () {
    test('fromJson parses all fields', () {
      final json = {
        'type': 'claude',
        'description': 'Anthropic model',
        'supportedRoles': ['writer', 'coder'],
        'isAuthenticated': true,
      };
      final provider = AgentProvider.fromJson(json);
      expect(provider.type, 'claude');
      expect(provider.description, 'Anthropic model');
      expect(provider.supportedRoles, ['writer', 'coder']);
      expect(provider.isAuthenticated, isTrue);
    });

    test('fromJson uses defaults for missing optional fields', () {
      final json = {'type': 'gemini'};
      final provider = AgentProvider.fromJson(json);
      expect(provider.type, 'gemini');
      expect(provider.description, '');
      expect(provider.supportedRoles, []);
      expect(provider.isAuthenticated, isFalse);
    });

    test('label returns correct string based on type', () {
      expect(AgentProvider(type: 'claude', description: '', supportedRoles: [], isAuthenticated: false).label, 'Claude (Anthropic)');
      expect(AgentProvider(type: 'gemini', description: '', supportedRoles: [], isAuthenticated: false).label, 'Gemini (Google)');
      expect(AgentProvider(type: 'openclaw', description: '', supportedRoles: [], isAuthenticated: false).label, 'OpenClaw');
      expect(AgentProvider(type: 'opencode', description: '', supportedRoles: [], isAuthenticated: false).label, 'OpenCode');
      expect(AgentProvider(type: 'ironclaw', description: '', supportedRoles: [], isAuthenticated: false).label, 'IronClaw');
      expect(AgentProvider(type: 'builtin', description: '', supportedRoles: [], isAuthenticated: false).label, 'Built-in');
      expect(AgentProvider(type: 'custom', description: '', supportedRoles: [], isAuthenticated: false).label, 'CUSTOM');
    });
  });
}
