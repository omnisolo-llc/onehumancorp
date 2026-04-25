import 'package:flutter/material.dart';

class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_nav': 'View your main business metrics and recent activity.',
    'agents_nav': 'Manage your AI agents and their assignments.',
    'tasks_nav': 'View tasks shared between you and your AI agents.',
    'chat_nav': 'Talk to your agents or customers directly.',
    'settings_nav': 'Configure your business details and preferences.',
    'help_fab': 'Ask our AI Help Agent any questions about OneHumanCorp.',
  };

  static String get(String key) {
    return _tooltips[key] ?? '';
  }
}

class ContextualTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;
  final String? customMessage;

  const ContextualTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
    this.customMessage,
  });

  @override
  Widget build(BuildContext context) {
    final message = customMessage ?? TooltipRegistry.get(tooltipKey);

    if (message.isEmpty) {
      return child;
    }

    return Tooltip(
      message: message,
      padding: const EdgeInsets.all(12),
      margin: const EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
        boxShadow: const [
          BoxShadow(
            color: Colors.black26,
            blurRadius: 8,
            offset: Offset(0, 4),
          ),
        ],
      ),
      textStyle: const TextStyle(
        color: Colors.white,
        fontFamily: 'Outfit',
        fontSize: 14,
        fontWeight: FontWeight.w500,
      ),
      preferBelow: true,
      verticalOffset: 24,
      triggerMode: TooltipTriggerMode.longPress, // Good for mobile
      child: child,
    );
  }
}
