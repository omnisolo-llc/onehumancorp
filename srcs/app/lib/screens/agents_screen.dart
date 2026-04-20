import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
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
      itemBuilder: (_, i) => _AnimatedAgentCard(
        key: ValueKey(agents[i].id ?? agents[i].name),
        agent: agents[i],
        index: i,
      ),
    );
  }
}

class _AnimatedAgentCard extends StatefulWidget {
  final Agent agent;
  final int index;
  const _AnimatedAgentCard({super.key, required this.agent, required this.index});

  @override
  State<_AnimatedAgentCard> createState() => _AnimatedAgentCardState();
}

class _AnimatedAgentCardState extends State<_AnimatedAgentCard> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<Offset> _slideAnimation;
  late Animation<double> _fadeAnimation;
  bool _isHovered = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 600),
    );
    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 0.2),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeOutQuart));
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0)
        .animate(CurvedAnimation(parent: _controller, curve: Curves.easeOut));

    Future.delayed(Duration(milliseconds: 100 * widget.index), () {
      if (mounted) {
        _controller.forward();
      }
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final isRunningColor =
        widget.agent.isRunning
            ? colorScheme.primary
            : colorScheme.surfaceContainerHighest;
    final isRunningIconColor =
        widget.agent.isRunning ? colorScheme.onPrimary : colorScheme.onSurfaceVariant;
    final chipBgColor =
        widget.agent.isRunning
            ? colorScheme.primaryContainer
            : colorScheme.surfaceContainerHighest;
    final chipTextColor =
        widget.agent.isRunning
            ? colorScheme.onPrimaryContainer
            : colorScheme.onSurfaceVariant;

    return Semantics(
      label: 'Agent ${widget.agent.name}, Role: ${widget.agent.role}, Status: ${widget.agent.status}',
      button: true,
      child: SlideTransition(
        position: _slideAnimation,
        child: FadeTransition(
          opacity: _fadeAnimation,
          child: Padding(
            padding: const EdgeInsets.only(bottom: 12),
            child: GlassCard(
        child: InkWell(
                        borderRadius: BorderRadius.circular(16),
                        onTap: () {
                          // Optional: Handle agent card tap
                        },
                        child: Tooltip(
                          message: 'View details for ${widget.agent.name}',
                          child: Padding(
                            padding: const EdgeInsets.all(16.0),
                            child: Row(
                              children: [
                                AnimatedContainer(
                                  duration: const Duration(milliseconds: 300),
                                  curve: Curves.easeInOut,
                                  width: 48,
                                  height: 48,
                                  alignment: Alignment.center,
                                  decoration: BoxDecoration(
                                    shape: BoxShape.circle,
                                    color: isRunningColor.withValues(alpha: 0.8),
                                  ),
                                  child: Icon(Icons.smart_toy, color: isRunningIconColor),
                                ),
                                const SizedBox(width: 16),
                                Expanded(
                                  child: Column(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    children: [
                                      Row(
                                        mainAxisSize: MainAxisSize.min,
                                        children: [
                                          Text(
                                            widget.agent.name,
                                            style: TextStyle(
                                              fontWeight: FontWeight.bold,
                                              fontSize: 16,
                                              color: colorScheme.onSurface,
                                              fontFamily: 'Outfit',
                                            ),
                                          ),
                                          if (widget.agent.svidVerified) ...[
                                            const SizedBox(width: 6),
                                            Tooltip(
                                              message: 'SPIFFE mTLS Secured',
                                              child: Icon(
                                                Icons.verified_user,
                                                size: 16,
                                                color: Colors.greenAccent,
                                              ),
                                            ),
                                          ],
                                        ],
                                      ),
                                      const SizedBox(height: 4),
                                      Text(
                                        widget.agent.formattedRole,
                                        style: TextStyle(
                                          color: colorScheme.onSurfaceVariant,
                                          fontFamily: 'Inter',
                                          fontSize: 14,
                                        ),
                                      ),
                                    ],
                                  ),
                                ),
                                const SizedBox(width: 16),
                                AnimatedContainer(
                                  duration: const Duration(milliseconds: 300),
                                  curve: Curves.easeInOut,
                                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                                  decoration: BoxDecoration(
                                    color: chipBgColor.withValues(alpha: 0.8),
                                    borderRadius: BorderRadius.circular(16),
                                  ),
                                  child: Text(
                                    widget.agent.status,
                                    style: TextStyle(
                                      color: chipTextColor,
                                      fontWeight: FontWeight.w600,
                                      fontSize: 12,
                                    ),
                                  ),
                                ),
                                if (!widget.agent.isRunning) ...[
                                  const SizedBox(width: 8),
                                  FilledButton.tonalIcon(
                                    onPressed: () => context.go('/wizards/fix/${widget.agent.id ?? widget.agent.name}'),
                                    icon: const Icon(Icons.build, size: 16),
                                    label: const Text('Help me fix this'),
                                    style: FilledButton.styleFrom(
                                      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 0),
                                      minimumSize: const Size(0, 32),
                                    ),
                                  ),
                                ],
                              ],
                            ),
                          ),
                        ),
                      ),
      ),
          ),
        ),
      ),
    );
  }
}
