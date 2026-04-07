import 'dart:async';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:ohc_app/models/agent_status.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class AgentSwarmDashboard extends StatefulWidget {
  const AgentSwarmDashboard({super.key});

  @override
  State<AgentSwarmDashboard> createState() => _AgentSwarmDashboardState();
}

class _AgentSwarmDashboardState extends State<AgentSwarmDashboard> {
  final List<AgentStatus> _agents = [
    AgentStatus(id: '1', name: 'Jules (Implementer)', currentTask: 'Awaiting tasks', state: AgentState.idle),
    AgentStatus(id: '2', name: 'Palette (Canvas)', currentTask: 'Awaiting tasks', state: AgentState.idle),
    AgentStatus(id: '3', name: 'Nova (Architect)', currentTask: 'Awaiting tasks', state: AgentState.idle),
    AgentStatus(id: '4', name: 'Scribe (DocGen)', currentTask: 'Awaiting tasks', state: AgentState.idle),
  ];

  final Random _random = Random();
  late Timer _timer;

  final List<String> _workingTasks = [
    'Drafting PR for UI components',
    'Running Bazel tests',
    'Analyzing pgvector memories',
    'Refactoring API docs',
    'Reviewing code changes',
    'Synchronizing local-to-cloud state',
    'Gathering market audit context',
  ];

  @override
  void initState() {
    super.initState();
    _timer = Timer.periodic(const Duration(seconds: 4), (timer) {
      if (!mounted) return;
      setState(() {
        final agentIndex = _random.nextInt(_agents.length);
        final stateRoll = _random.nextDouble();

        AgentState newState;
        String newTask;

        if (stateRoll < 0.1) {
          newState = AgentState.blocked;
          newTask = 'Blocked: Missing requirements';
        } else if (stateRoll < 0.4) {
          newState = AgentState.idle;
          newTask = 'Awaiting tasks';
        } else {
          newState = AgentState.working;
          newTask = _workingTasks[_random.nextInt(_workingTasks.length)];
        }

        _agents[agentIndex] = _agents[agentIndex].copyWith(
          state: newState,
          currentTask: newTask,
        );
      });
    });
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Agent Swarm Dashboard',
          style: Theme.of(context).textTheme.titleLarge?.copyWith(
            fontWeight: FontWeight.bold,
            fontFamily: 'Outfit',
          ),
        ),
        const SizedBox(height: 16),
        ListView.separated(
          shrinkWrap: true,
          physics: const NeverScrollableScrollPhysics(),
          itemCount: _agents.length,
          separatorBuilder: (context, index) => const SizedBox(height: 12),
          itemBuilder: (context, index) {
            final agent = _agents[index];
            return _AgentCard(key: ValueKey(agent.id), agent: agent);
          },
        ),
      ],
    );
  }
}

class _AgentCard extends StatelessWidget {
  final AgentStatus agent;

  const _AgentCard({super.key, required this.agent});

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final color = _getStatusColor(agent.state);

    return GlassCard(
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
      child: Row(
        children: [
          CircleAvatar(
            backgroundColor: colors.primaryContainer,
            child: Text(
              agent.name.substring(0, 1),
              style: TextStyle(color: colors.onPrimaryContainer),
            ),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  agent.name,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Outfit',
                    fontSize: 16,
                  ),
                ),
                const SizedBox(height: 4),
                AnimatedSwitcher(
                  duration: const Duration(milliseconds: 300),
                  transitionBuilder: (Widget child, Animation<double> animation) {
                    return FadeTransition(
                      opacity: animation,
                      child: SlideTransition(
                        position: Tween<Offset>(
                          begin: const Offset(0.0, 0.2),
                          end: Offset.zero,
                        ).animate(animation),
                        child: child,
                      ),
                    );
                  },
                  child: Text(
                    agent.currentTask,
                    key: ValueKey<String>(agent.currentTask),
                    style: TextStyle(
                      fontFamily: 'Inter',
                      fontSize: 14,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(width: 16),
          _StatusDot(color: color, isWorking: agent.state == AgentState.working),
        ],
      ),
    );
  }

  Color _getStatusColor(AgentState state) {
    switch (state) {
      case AgentState.working:
        return Colors.greenAccent;
      case AgentState.idle:
        return Colors.grey;
      case AgentState.blocked:
        return Colors.redAccent;
    }
  }
}

class _StatusDot extends StatefulWidget {
  final Color color;
  final bool isWorking;

  const _StatusDot({required this.color, required this.isWorking});

  @override
  State<_StatusDot> createState() => _StatusDotState();
}

class _StatusDotState extends State<_StatusDot> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 1),
    );
    _pulseAnimation = Tween<double>(begin: 1.0, end: 1.5).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );

    if (widget.isWorking) {
      _controller.repeat(reverse: true);
    }
  }

  @override
  void didUpdateWidget(covariant _StatusDot oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isWorking && !oldWidget.isWorking) {
      _controller.repeat(reverse: true);
    } else if (!widget.isWorking && oldWidget.isWorking) {
      _controller.stop();
      _controller.value = 0.0;
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _pulseAnimation,
      builder: (context, child) {
        return Transform.scale(
          scale: widget.isWorking ? _pulseAnimation.value : 1.0,
          child: Container(
            width: 12,
            height: 12,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: widget.color,
              boxShadow: widget.isWorking
                  ? [
                      BoxShadow(
                        color: widget.color.withValues(alpha: 0.5),
                        blurRadius: 8,
                        spreadRadius: 2,
                      )
                    ]
                  : null,
            ),
          ),
        );
      },
    );
  }
}
