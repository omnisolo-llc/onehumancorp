import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/tooltip_registry.dart';

class ContextualTooltip extends ConsumerWidget {
  final Widget child;
  final String tooltipKey;
  final String? fallbackMessage;

  const ContextualTooltip({
    super.key,
    required this.child,
    required this.tooltipKey,
    this.fallbackMessage,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final registry = ref.watch(tooltipRegistryProvider);
    final message = registry[tooltipKey] ?? fallbackMessage ?? 'No help available.';

    return Tooltip(
      message: message,
      triggerMode: TooltipTriggerMode.longPress, // Good for mobile
      decoration: BoxDecoration(
        color: const Color(0xFF1E293B).withAlpha(240),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withAlpha(25)),
      ),
      textStyle: const TextStyle(
        fontFamily: 'Inter',
        fontSize: 14,
        color: Colors.white,
      ),
      padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 12),
      margin: const EdgeInsets.symmetric(horizontal: 20),
      child: child,
    );
  }
}
