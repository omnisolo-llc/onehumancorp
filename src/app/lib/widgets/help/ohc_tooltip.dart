import 'package:flutter/material.dart';
import 'tooltip_registry.dart';

class OhcTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const OhcTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final tooltipText = TooltipRegistry.getTooltip(tooltipKey);

    if (tooltipText == null) {
      return child; // Fallback if key not found
    }

    return Tooltip(
      message: tooltipText,
      padding: const EdgeInsets.all(12),
      margin: const EdgeInsets.symmetric(horizontal: 16),
      textStyle: const TextStyle(
        color: Colors.white,
        fontFamily: 'Inter',
        fontSize: 14,
      ),
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
      ),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
