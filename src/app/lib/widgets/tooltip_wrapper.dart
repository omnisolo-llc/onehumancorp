import 'package:flutter/material.dart';
import '../help_registry.dart';

class TooltipWrapper extends StatelessWidget {
  final String tooltipId;
  final Widget child;
  final AxisDirection preferDirection;

  const TooltipWrapper({
    Key? key,
    required this.tooltipId,
    required this.child,
    this.preferDirection = AxisDirection.down,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final tooltipText = HelpRegistry().getTooltip(tooltipId);

    if (tooltipText == null || tooltipText.isEmpty) {
      return child;
    }

    return Tooltip(
      message: tooltipText,
      triggerMode: TooltipTriggerMode.longPress, // For mobile long-press and desktop hover
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
      ),
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        fontSize: 14,
        color: Colors.white,
      ),
      padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 12),
      margin: const EdgeInsets.symmetric(horizontal: 16),
      preferBelow: preferDirection == AxisDirection.down,
      showDuration: const Duration(seconds: 3),
      child: child,
    );
  }
}
