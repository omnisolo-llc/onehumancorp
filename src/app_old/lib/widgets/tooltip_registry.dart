import 'package:flutter/material.dart';

class TooltipRegistry {
  static final Map<String, String> _tooltips = {
    'dashboard_revenue': 'Total money collected from sales and bookings.',
    'dashboard_my_business': 'An overview of your core business metrics and status.',
    'dashboard_tasks': 'Automated tasks your AI agents are currently working on.',
    'agents_list': 'Your virtual team members who handle specific parts of your business.',
    'billing_plan': 'Your current subscription plan and usage limits.',
    'ai_settings': 'Configure how your AI agents behave and what access they have.',
  };

  static String get(String key) => _tooltips[key] ?? 'Help information is not available.';
}

class ContextualTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const ContextualTooltip({super.key, required this.tooltipKey, required this.child});

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: TooltipRegistry.get(tooltipKey),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
