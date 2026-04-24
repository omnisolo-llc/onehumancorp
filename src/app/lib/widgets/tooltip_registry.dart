import 'package:flutter/material.dart';

class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_growth': 'See how fast your business is growing and discover what is working best.',
    'dashboard_ai_agents': 'Manage your AI assistants. Hire or fire agents to automate different parts of your business.',
    'dashboard_recent_activity': 'Your recent business events like new orders, bookings, and messages.',
    'help_center_search': 'Type a question or keywords to find articles, guides, and tutorials.',
    'help_center_chat': 'Click to open the AI Support Agent. It can answer any question about running your business.',
    'help_center_video': 'Watch a short tutorial video showing exactly how to use this feature.',
  };

  static String getTooltip(String key) {
    return _tooltips[key] ?? 'More information';
  }
}

class ContextualTooltip extends StatelessWidget {
  final Widget child;
  final String tooltipKey;
  final String? customMessage;

  const ContextualTooltip({
    super.key,
    required this.child,
    required this.tooltipKey,
    this.customMessage,
  });

  @override
  Widget build(BuildContext context) {
    final message = customMessage ?? TooltipRegistry.getTooltip(tooltipKey);
    return Tooltip(
      message: message,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      margin: const EdgeInsets.symmetric(horizontal: 16),
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
        fontSize: 14,
        fontWeight: FontWeight.w500,
      ),
      decoration: BoxDecoration(
        color: const Color.fromRGBO(20, 20, 25, 0.85),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withValues(alpha: 0.15)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.5),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
