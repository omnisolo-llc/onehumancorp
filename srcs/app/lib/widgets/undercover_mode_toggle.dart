import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

final undercoverModeProvider = StateProvider<bool>((ref) => false);

class UndercoverModeToggle extends ConsumerWidget {
  const UndercoverModeToggle({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isUndercover = ref.watch(undercoverModeProvider);
    final theme = Theme.of(context);

    return Semantics(
      label: 'Undercover Mode Toggle',
      toggled: isUndercover,
      child: Tooltip(
        message: 'Toggle Undercover Mode (Glassmorphism & Saturation)',
        child: InkWell(
          onTap: () => ref.read(undercoverModeProvider.notifier).state = !isUndercover,
          borderRadius: BorderRadius.circular(16),
          child: GlassCard(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Icon(
                  isUndercover ? Icons.visibility_off : Icons.visibility,
                  color: isUndercover ? theme.colorScheme.primary : theme.colorScheme.onSurfaceVariant,
                ),
                const SizedBox(width: 12),
                Text(
                  'Undercover Mode',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.bold,
                    color: isUndercover ? theme.colorScheme.primary : theme.colorScheme.onSurface,
                  ),
                ),
                const SizedBox(width: 16),
                Switch(
                  value: isUndercover,
                  onChanged: (val) => ref.read(undercoverModeProvider.notifier).state = val,
                  activeColor: theme.colorScheme.primary,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
