import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/help_service.dart';

class ContextualTooltip extends ConsumerWidget {
  final String elementId;
  final Widget child;

  const ContextualTooltip({
    super.key,
    required this.elementId,
    required this.child,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tooltips = ref.watch(tooltipsProvider);

    return tooltips.when(
      data: (map) {
        final message = map[elementId];
        if (message == null) return child;
        return Tooltip(
          message: message,
          padding: const EdgeInsets.all(12),
          margin: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceContainerHighest.withValues(alpha: 0.9),
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.5)),
          ),
          textStyle: TextStyle(
            color: Theme.of(context).colorScheme.onSurface,
            fontFamily: 'Inter',
            fontSize: 13,
          ),
          child: child,
        );
      },
      loading: () => child,
      error: (_, __) => child,
    );
  }
}
