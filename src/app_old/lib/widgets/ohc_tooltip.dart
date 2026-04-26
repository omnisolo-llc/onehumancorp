import 'package:flutter/material.dart';

class OhcTooltip extends StatelessWidget {
  final String message;
  final Widget child;

  const OhcTooltip({
    super.key,
    required this.message,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: message,
      textStyle: const TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: Colors.white,
      ),
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.all(12),
      margin: const EdgeInsets.all(12),
      preferBelow: true,
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
