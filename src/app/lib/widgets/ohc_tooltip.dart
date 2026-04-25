import 'package:ohc_app/widgets/tooltip_registry.dart';
import 'package:flutter/material.dart';

class OhcTooltip extends StatelessWidget {
  final String message;
  final String? registryKey;
  final Widget child;

  const OhcTooltip({
    super.key,
    this.message = '',
    this.registryKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final displayMessage = registryKey != null ? TooltipRegistry.get(registryKey!) : message;
    return Tooltip(
      message: displayMessage,
      padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 12.0),
      margin: const EdgeInsets.all(8.0),
      textStyle: const TextStyle(
        color: Colors.white,
        fontWeight: FontWeight.w500,
        fontFamily: 'Outfit',
        fontSize: 14.0,
      ),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.9),
        borderRadius: BorderRadius.circular(8.0),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.1),
            blurRadius: 8.0,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      preferBelow: false,
      triggerMode: TooltipTriggerMode.longPress, // Long-press on mobile, hover on desktop handled by default
      child: child,
    );
  }
}
