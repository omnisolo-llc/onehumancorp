import 'package:flutter/material.dart';

class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'observability_panel': 'View your system health and see what your agents are currently doing.',
    'scale_role': 'Change how many agents you have for this role.',
    'decrease_agent': 'Remove one agent from this role.',
    'increase_agent': 'Add one more agent to this role.',
    'stat_card': 'View more details about this metric.',
    'help_center': 'Open the Help Center to find guides and tutorials.',
    'chat_fab': 'Ask our AI Help Agent a question or request a tutorial.',
  };

  static String get(String key, {String? fallback}) {
    return _tooltips[key] ?? fallback ?? 'Learn more about this feature.';
  }
}

class OhcTooltip extends StatelessWidget {
  final String registryKey;
  final String? fallbackMessage;
  final Widget child;

  const OhcTooltip({
    super.key,
    required this.registryKey,
    required this.child,
    this.fallbackMessage,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: TooltipRegistry.get(registryKey, fallback: fallbackMessage),
      textStyle: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 14),
      decoration: BoxDecoration(color: Colors.black87, borderRadius: BorderRadius.circular(8)),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
