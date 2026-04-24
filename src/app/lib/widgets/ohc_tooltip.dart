import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/tooltip_registry.dart';

class OhcTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;
  final String? fallback;

  const OhcTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
    this.fallback,
  });

  @override
  Widget build(BuildContext context) {
    final message = TooltipRegistry.get(tooltipKey);
    final text = message.isNotEmpty ? message : (fallback ?? '');

    if (text.isEmpty) {
      return child;
    }

    return Tooltip(
      message: text,
      textStyle: const TextStyle(
        fontFamily: 'Outfit',
        fontFamilyFallback: ['Inter'],
        color: Colors.white,
        fontSize: 12,
      ),
      decoration: BoxDecoration(
        color: const Color(0xFF1E1E1E).withOpacity(0.9), // Dark glass
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withOpacity(0.1)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.2),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      margin: const EdgeInsets.all(8),
      preferBelow: true,
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
