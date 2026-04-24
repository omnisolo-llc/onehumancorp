import 'package:flutter/material.dart';

class TooltipRegistry {
  static const Map<String, String> _registry = {
    'dashboard_hire_agent': 'Click here to hire a new AI agent to join your team.',
    'dashboard_refresh': 'Refresh the dashboard to see the latest data.',
    'dashboard_help': 'Open the Help Center for guides and support.',
    'task_list_new': 'Create a new shared task for the swarm.',
    'chat_send': 'Send your message to the agent or room.',
  };

  static String getTooltip(String key) {
    return _registry[key] ?? 'Help tooltip not found for $key';
  }
}

class HelpTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const HelpTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: TooltipRegistry.getTooltip(tooltipKey),
      waitDuration: const Duration(milliseconds: 500),
      textStyle: const TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: Colors.white,
      ),
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white24),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      child: child,
    );
  }
}
