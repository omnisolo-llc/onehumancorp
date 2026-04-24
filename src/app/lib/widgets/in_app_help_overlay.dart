import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:ohc_app/widgets/ohc_tooltip.dart';

class InAppHelpOverlay extends StatelessWidget {
  final Widget child;

  const InAppHelpOverlay({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        child,
        Positioned(
          bottom: 24,
          right: 24,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              OhcTooltip(
                tooltipKey: 'help_center',
                child: FloatingActionButton.small(
                  heroTag: 'help_center_fab',
                  onPressed: () => context.go('/help'),
                  backgroundColor: Theme.of(context).colorScheme.surfaceContainerHighest,
                  child: Icon(Icons.help_outline, color: Theme.of(context).colorScheme.onSurface),
                ),
              ),
              const SizedBox(height: 12),
              OhcTooltip(
                tooltipKey: 'ask_anything',
                child: FloatingActionButton(
                  heroTag: 'help_chat_fab',
                  onPressed: () => context.go('/help/chat'),
                  backgroundColor: Theme.of(context).colorScheme.primary,
                  child: const Icon(Icons.support_agent, color: Colors.white),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
