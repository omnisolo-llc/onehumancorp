import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/models/agent.dart';
import 'package:ohc_app/services/api_service.dart';

final _agentsProvider = FutureProvider<List<Agent>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  return api.listAgents();
});

class AgentsScreen extends ConsumerWidget {
  const AgentsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final snapshot = ref.watch(_agentsProvider);
    return Scaffold(
      appBar: AppBar(
        title: const Text('Agents'),
        actions: [
          Tooltip(
            message: 'Hire a new agent',
            child: FilledButton.icon(
              onPressed: () => context.go('/agents/hire'),
              icon: const Icon(Icons.add),
              label: const Text('Hire Agent'),
            ),
          ),
          const SizedBox(width: 16),
        ],
      ),
      body: snapshot.when(
        loading:
            () => Center(
              child: CircularProgressIndicator(
                color: Theme.of(context).colorScheme.primary,
              ),
            ),
        error:
            (e, _) => Center(
              child: Text(
                'Error: $e',
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ),
        data:
            (agents) =>
                agents.isEmpty
                    ? _EmptyAgents(onHire: () => context.go('/agents/hire'))
                    : _AgentList(agents: agents),
      ),
    );
  }
}

class _EmptyAgents extends StatelessWidget {
  final VoidCallback onHire;
  const _EmptyAgents({required this.onHire});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            Icons.smart_toy,
            size: 64,
            color: Theme.of(context).colorScheme.outline,
          ),
          const SizedBox(height: 16),
          Text('No agents yet', style: Theme.of(context).textTheme.titleLarge),
          const SizedBox(height: 8),
          const Text('Hire your first AI agent to get started.'),
          const SizedBox(height: 24),
          Tooltip(
            message: 'Hire a new agent to your team',
            child: FilledButton.icon(
              onPressed: onHire,
              icon: const Icon(Icons.add),
              label: const Text('Hire New Agent'),
            ),
          ),
        ],
      ),
    );
  }
}

class _AgentList extends StatelessWidget {
  final List<Agent> agents;
  const _AgentList({required this.agents});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: agents.length,
      itemBuilder: (_, i) => _AgentCard(agent: agents[i]),
    );
  }
}

class _AgentCard extends StatelessWidget {
  final Agent agent;
  const _AgentCard({required this.agent});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final isRunningColor =
        agent.isRunning
            ? colorScheme.primary
            : colorScheme.surfaceContainerHighest;
    final isRunningIconColor =
        agent.isRunning ? colorScheme.onPrimary : colorScheme.onSurfaceVariant;
    final chipBgColor =
        agent.isRunning
            ? colorScheme.primaryContainer
            : colorScheme.surfaceContainerHighest;
    final chipTextColor =
        agent.isRunning
            ? colorScheme.onPrimaryContainer
            : colorScheme.onSurfaceVariant;

    return Semantics(
      excludeSemantics: true,
      label:
          'Agent ${agent.name}, Role: ${agent.role}, Status: ${agent.status}',
      child: Card(
        margin: const EdgeInsets.only(bottom: 12),
        elevation: 0,
        color: colorScheme.surface,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: BorderSide(color: colorScheme.outlineVariant, width: 1),
        ),
        child: InkWell(
          borderRadius: BorderRadius.circular(12),
          onTap: () {
            // Optional: Handle agent card tap
          },
          child: Tooltip(
            message: 'View details for ${agent.name}',
            child: Padding(
              padding: const EdgeInsets.symmetric(vertical: 4.0),
              child: ListTile(
                leading: CircleAvatar(
                  backgroundColor: isRunningColor,
                  child: Icon(Icons.smart_toy, color: isRunningIconColor),
                ),
                title: Text(
                  agent.name,
                  style: TextStyle(
                    fontWeight: FontWeight.w600,
                    color: colorScheme.onSurface,
                  ),
                ),
                subtitle: Text(
                  agent.role,
                  style: TextStyle(color: colorScheme.onSurfaceVariant),
                ),
                trailing: Chip(
                  label: Text(
                    agent.status,
                    style: TextStyle(color: chipTextColor),
                  ),
                  backgroundColor: chipBgColor,
                  side: BorderSide.none,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
