import 'package:flutter/material.dart';

class TooltipService {
  static final Map<String, String> _registry = {
    'dashboard_revenue': 'Your total revenue for the current month.',
    'dashboard_ai_agents': 'Status of your active AI assistants.',
    'dashboard_active_orders': 'Orders currently being processed.',
    'dashboard_new_messages': 'Unread messages from customers.',
    'dashboard_settings': 'Configure your store and AI agents.',
  };

  static String getTooltipText(String key) {
    return _registry[key] ?? '';
  }

  static void setTooltipText(String key, String text) {
    _registry[key] = text;
  }
}

class TooltipWrapper extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const TooltipWrapper({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final text = TooltipService.getTooltipText(tooltipKey);
    if (text.isEmpty) {
      return child;
    }

    return Tooltip(
      message: text,
      preferBelow: false,
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.8),
        borderRadius: BorderRadius.circular(8),
      ),
      textStyle: const TextStyle(
        color: Colors.white,
        fontSize: 14,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
      margin: const EdgeInsets.all(8),
      showDuration: const Duration(seconds: 3),
      child: child,
    );
  }
}