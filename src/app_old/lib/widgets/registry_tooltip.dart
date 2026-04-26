import 'package:flutter/material.dart';
import '../services/tooltip_registry.dart';

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
      child: child,
    );
  }
}
