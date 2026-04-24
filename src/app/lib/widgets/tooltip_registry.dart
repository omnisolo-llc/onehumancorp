import 'package:flutter/material.dart';

/// A registry for all tooltips used across the application.
/// This allows non-technical team members (like the Scribe agent) to update
/// tooltip text without digging through UI code.
class TooltipRegistry {
  static const Map<String, String> _tooltips = {
    'dashboard_refresh': 'Update the information shown on your dashboard.',
    'dashboard_hire_agent': 'Add a new AI teammate to your business.',
    'dashboard_fire_agent': 'Remove this AI teammate from your business.',
    'dashboard_system_health': 'Shows if all your systems and agents are running smoothly.',
    'chat_switch_room': 'Change which conversation you are looking at.',
    'chat_send_message': 'Send your message to the agent or customer.',
    'user_refresh': 'Update the list of users.',
    'user_delete': 'Permanently remove this user from your business.',
    'login_settings': 'Advanced options for connecting to the server.',
    'login_show_password': 'Show or hide your password.',
    'settings_backend_url': 'Change the server address (advanced).',
    'help_button': 'Open the Help Center to find answers or ask our AI.',
  };

  /// Retrieves the tooltip text for a given key.
  /// If the key is not found, it returns a default message or the key itself
  /// to help identify missing translations during development.
  static String get(String key) {
    if (_tooltips.containsKey(key)) {
      return _tooltips[key]!;
    }
    debugPrint('Warning: Tooltip key "$key" not found in registry.');
    return 'Tooltip: $key';
  }
}

/// A wrapper around Flutter's Tooltip that uses the registry.
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
      message: TooltipRegistry.get(tooltipKey),
      // Prefer long press for mobile, hover for desktop
      triggerMode: TooltipTriggerMode.longPress,
      child: child,
    );
  }
}
