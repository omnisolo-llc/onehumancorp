import 'package:flutter/material.dart';

/// A standardized tooltip registry to ensure plain-language text and
/// consistent OHC styling across the entire app.
class TooltipRegistry extends StatelessWidget {
  final String message;
  final Widget child;

  const TooltipRegistry({
    super.key,
    required this.message,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: message,
      waitDuration: const Duration(milliseconds: 500),
      textStyle: const TextStyle(
        fontSize: 14,
        color: Colors.white,
        fontFamily: 'Outfit',
      ),
      decoration: BoxDecoration(
        color: Colors.black87,
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      child: child,
    );
  }
}
