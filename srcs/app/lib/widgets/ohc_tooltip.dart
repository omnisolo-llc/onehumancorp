import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';

class OhcTooltip extends StatelessWidget {
  final Widget child;
  final String message;

  const OhcTooltip({
    super.key,
    required this.child,
    required this.message,
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
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.8),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withOpacity(0.1)),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      margin: const EdgeInsets.all(8),
      waitDuration: const Duration(milliseconds: 500),
      showDuration: const Duration(seconds: 3),
      triggerMode: TooltipTriggerMode.longPress,
      child: child,
    );
  }
}
