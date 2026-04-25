import 'package:flutter/material.dart';

/// A registry for all tooltips in the application.
/// This allows updating tooltip text without touching UI code.
class TooltipRegistry {
  static final Map<String, String> _tooltips = {
    'dashboard_scale_agents': 'Manage the number of AI agents allocated to this role.',
    'task_list_dependencies': 'Tasks that must be completed before this one can start.',
    'task_list_workflow_state': 'The current internal state of the agent handling this task.',
    'sidebar_help_center': 'Find guides, tutorials, and support for your business.',
    'api_config_key': 'Your secure provider API key. Stored encrypted.',
  };

  /// Retrieve a tooltip by key. Returns a fallback or empty string if not found.
  static String get(String key) {
    return _tooltips[key] ?? '';
  }

  /// Add or update a tooltip programmatically (e.g. from an API).
  static void update(String key, String message) {
    _tooltips[key] = message;
  }
}

/// A custom Tooltip widget that uses the OHC TooltipRegistry.
/// It uses a plain language approach with a clean, semi-transparent design.
class OhcTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;
  final String? fallbackMessage;

  const OhcTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
    this.fallbackMessage,
  });

  @override
  Widget build(BuildContext context) {
    final message = TooltipRegistry.get(tooltipKey);
    final displayMessage = message.isNotEmpty ? message : (fallbackMessage ?? '');

    if (displayMessage.isEmpty) {
      return child;
    }

    return Tooltip(
      message: displayMessage,
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
        fontSize: 14,
      ),
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white24),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
