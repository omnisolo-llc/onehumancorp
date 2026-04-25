import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/help/tooltip_registry.dart';

class ContextualTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;
  final bool preferBelow;

  const ContextualTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
    this.preferBelow = true,
  });

  @override
  Widget build(BuildContext context) {
    final message = TooltipRegistry.get(tooltipKey);

    // If the key is not found, just return the child without a tooltip
    if (message.isEmpty) {
      return child;
    }

    return Tooltip(
      message: message,
      preferBelow: preferBelow,
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
        fontSize: 14,
      ),
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.8),
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      margin: const EdgeInsets.all(8),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
