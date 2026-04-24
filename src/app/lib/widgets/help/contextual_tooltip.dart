import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/help/tooltip_registry.dart';

class ContextualTooltip extends StatelessWidget {
  final String id;
  final Widget child;

  const ContextualTooltip({
    super.key,
    required this.id,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final message = TooltipRegistry.get(id);

    return Tooltip(
      message: message,
      textStyle: const TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: Colors.white,
      ),
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.8),
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.all(12),
      margin: const EdgeInsets.symmetric(horizontal: 16),
      waitDuration: const Duration(milliseconds: 500),
      showDuration: const Duration(seconds: 3),
      triggerMode: TooltipTriggerMode.longPress,
      child: child,
    );
  }
}
