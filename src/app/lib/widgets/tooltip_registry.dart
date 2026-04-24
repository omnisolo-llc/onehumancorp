import 'package:flutter/material.dart';

/// A singleton registry for tooltips across the app.
class TooltipRegistry {
  static final TooltipRegistry _instance = TooltipRegistry._internal();
  factory TooltipRegistry() => _instance;
  TooltipRegistry._internal();

  final Map<String, String> _tooltips = {
    'dashboard_scale_role': 'Change the number of active agents for this role.',
    'dashboard_decrease_agent': 'Remove one agent to save costs.',
    'dashboard_increase_agent': 'Add one agent to handle more tasks.',
    'dashboard_stat_card': 'View more details about this metric.',
    'login_connection_settings': 'Configure connection settings.',
    'login_show_password': 'Show password text.',
    'login_hide_password': 'Hide password text.',
  };

  /// Register or update a tooltip.
  void register(String key, String text) {
    _tooltips[key] = text;
  }

  /// Look up a tooltip by key. Returns the key itself if not found.
  String lookup(String key) {
    return _tooltips[key] ?? key;
  }
}

/// A wrapper around standard Tooltip that uses the TooltipRegistry.
class RegisteredTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const RegisteredTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: TooltipRegistry().lookup(tooltipKey),
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
        fontSize: 14,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        color: const Color.fromRGBO(30, 30, 30, 0.95),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
      ),
      child: child,
    );
  }
}
