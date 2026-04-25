import 'package:flutter/material.dart';

/// A central registry for plain-language tooltips used across the OHC app.
/// This allows technical writers to update tooltips without touching UI code.
class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'ai_agent_status': 'Shows if your AI Agent is working or sleeping. Green means active.',
    'dashboard_revenue': 'Your total revenue from all sources over the last 30 days. Tap for details.',
    'dashboard_referrals': 'How many new customers came from your referral links.',
    'dashboard_handoffs': 'Tasks your AI agents couldn\'t finish and need your help with.',
    'dashboard_agents': 'Manage your AI workforce. Hire new agents or adjust their settings.',
    'global_help': 'Open the Help Center to find answers and guides for your business.',
    'agent_skills': 'Abilities you can teach your AI agent to make them more helpful.',
    'security_scan': 'Check your business for any security risks. We do this automatically every day.',
  };

  /// Returns the registered tooltip text for the given key, or a fallback.
  static String get(String key, {String fallback = 'More information'}) {
    return _tooltips[key] ?? fallback;
  }
}

/// A convenient widget that wraps its child with a tooltip from the registry.
class RegisteredTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;
  final String? fallback;

  const RegisteredTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
    this.fallback,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: TooltipRegistry.get(tooltipKey, fallback: fallback ?? 'More information'),
      triggerMode: TooltipTriggerMode.longPress,
      preferBelow: false,
      child: child,
    );
  }
}
