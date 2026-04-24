import 'package:flutter/material.dart';

class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_health': 'System health score based on agent status.',
    'agent_hire': 'Hire a new agent for your swarm.',
    'agent_fire': 'Remove this agent from your swarm.',
    'agent_memory': 'View what this agent has learned and remembered.',
    'task_list': 'View shared tasks your agents are working on.',
    'meeting_room': 'Join or create a virtual meeting room for agents.',
    'api_key_hidden': 'Show or hide your API key.',
    'refresh_pipelines': 'Refresh the list of your pipelines.',
  };

  static String get(String key) {
    return _tooltips[key] ?? '';
  }
}

class RegistryTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const RegistryTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final message = TooltipRegistry.get(tooltipKey);
    if (message.isEmpty) {
      return child;
    }
    return Tooltip(
      message: message,
      child: child,
    );
  }
}
