import 'dart:ui';
import 'package:flutter/material.dart';

class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_health': 'View the overall health and metrics of your business system. A green indicator means everything is running smoothly.',
    'dashboard_mission': 'Missions are high-level goals you have given to your AI agents. This shows how many are currently active.',
    'dashboard_agent_count': 'This is the total number of AI agents currently working for your business.',
    'dashboard_scale_role': 'Adjust how many resources are allocated to this specific role. More resources mean tasks are completed faster.',
    'agent_status': 'Shows if the agent is currently working, idle, or needs your attention. Green means it is actively processing tasks.',
    'agent_memory': 'This agent can remember past interactions to provide better service. Click to view its memory log.',
    'help_button': 'Click here if you need assistance. Our AI Help Agent can answer any questions you have.',
    'setup_domain': 'Your domain is your web address (like www.yourbusiness.com). This is where customers will find your store.',
    'setup_payments': 'Connect your bank account so you can receive money from customer purchases securely.',
  };

  static String get(String key) {
    return _tooltips[key] ?? 'More information available soon.';
  }
}

class ContextTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const ContextTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final message = TooltipRegistry.get(tooltipKey);

    return Tooltip(
      message: message,
      textStyle: const TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: Colors.white,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      margin: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: Colors.black.withValues(alpha: 0.7),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.white.withValues(alpha: 0.2)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.2),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
