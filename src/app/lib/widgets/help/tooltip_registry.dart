import 'package:flutter/material.dart';

class OhcTooltip extends StatelessWidget {
  final String message;
  final Widget child;

  const OhcTooltip({
    super.key,
    required this.message,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: message,
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
        fontSize: 14,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      margin: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.9),
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.1),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      waitDuration: const Duration(milliseconds: 500),
      showDuration: const Duration(seconds: 3),
      triggerMode: TooltipTriggerMode.longPress, // Works on mobile via long press
      child: child,
    );
  }
}

// A simple registry if we need to centralize messages later
class TooltipRegistry {
  static const Map<String, String> messages = {
    'dashboard_hire': 'Hire a new AI agent to handle tasks automatically.',
    'dashboard_fire': 'Remove this AI agent from your team.',
    'dashboard_title': 'Your central hub to view your business health and manage operations.',
  };

  static String get(String key) => messages[key] ?? '';
}
