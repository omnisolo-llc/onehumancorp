import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final undercoverModeProvider = StateProvider<bool>((ref) => false);

class UndercoverModeToggle extends ConsumerWidget {
  const UndercoverModeToggle({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isUndercover = ref.watch(undercoverModeProvider);
    final theme = Theme.of(context);

    return Tooltip(
      message: 'Toggle Undercover Mode',
      child: Semantics(
        button: true,
        label: isUndercover ? 'Disable Undercover Mode' : 'Enable Undercover Mode',
        child: GestureDetector(
          onTap: () {
            ref.read(undercoverModeProvider.notifier).state = !isUndercover;
          },
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
            width: 60,
            height: 32,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(16),
              color: isUndercover
                  ? theme.colorScheme.primary.withValues(alpha: 0.3)
                  : theme.colorScheme.surface.withValues(alpha: 0.1),
              border: Border.all(
                color: isUndercover
                    ? theme.colorScheme.primary.withValues(alpha: 0.5)
                    : theme.colorScheme.onSurface.withValues(alpha: 0.2),
              ),
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                child: Stack(
                  children: [
                    AnimatedAlign(
                      duration: const Duration(milliseconds: 250),
                      curve: Curves.easeInOut,
                      alignment: isUndercover ? Alignment.centerRight : Alignment.centerLeft,
                      child: Container(
                        margin: const EdgeInsets.all(2),
                        width: 26,
                        height: 26,
                        decoration: BoxDecoration(
                          shape: BoxShape.circle,
                          color: isUndercover ? theme.colorScheme.primary : theme.colorScheme.onSurface,
                        ),
                        child: Icon(
                          isUndercover ? Icons.visibility_off : Icons.visibility,
                          size: 16,
                          color: isUndercover ? theme.colorScheme.onPrimary : theme.colorScheme.surface,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
