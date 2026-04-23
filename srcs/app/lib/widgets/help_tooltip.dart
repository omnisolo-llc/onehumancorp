import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/tooltip_registry.dart';

class HelpTooltip extends ConsumerWidget {
  final String tooltipKey;
  final Widget child;

  const HelpTooltip({
    super.key,
    required this.tooltipKey,
    required this.child,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final registry = ref.watch(tooltipRegistryProvider);
    final message = registry.getTooltip(tooltipKey);

    if (message.isEmpty) {
      return child;
    }

    return Tooltip(
      message: message,
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        color: Colors.white,
        fontSize: 14,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: const Color.fromRGBO(255, 255, 255, 0.05),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(alpha: 0.2),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
        // Note: Flutter Tooltip decoration doesn't directly support BackdropFilter,
        // so we use a semi-transparent background to approximate the glass effect.
      ),
      waitDuration: const Duration(milliseconds: 500),
      child: child,
    );
  }
}
