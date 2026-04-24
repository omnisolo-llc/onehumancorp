import 'package:flutter/material.dart';
import 'package:ohc_app/services/tooltip_registry.dart';

class RegistryTooltip extends StatelessWidget {
  final String tooltipKey;
  final Widget child;

  const RegistryTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: TooltipRegistry.get(tooltipKey),
      waitDuration: const Duration(milliseconds: 500),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface.withOpacity(0.9),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Theme.of(context).colorScheme.outline.withOpacity(0.2)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      textStyle: TextStyle(
        fontFamily: 'Inter',
        color: Theme.of(context).colorScheme.onSurface,
        fontSize: 14,
      ),
      child: child,
    );
  }
}
