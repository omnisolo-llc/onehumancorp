import 'package:flutter/material.dart';

class ContextualTooltip extends StatelessWidget {
  final String message;
  final Widget child;

  const ContextualTooltip({super.key, required this.message, required this.child});

  @override
  Widget build(BuildContext context) {
    // We use a plain-language tooltip overlay for non-obvious UI
    return Tooltip(
      message: message,
      textStyle: const TextStyle(fontFamily: 'Inter', color: Colors.white, fontSize: 14),
      decoration: BoxDecoration(
        color: const Color(0xFF1A1A33),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withAlpha(26)),
      ),
      padding: const EdgeInsets.all(12),
      showDuration: const Duration(seconds: 3),
      child: child,
    );
  }
}
