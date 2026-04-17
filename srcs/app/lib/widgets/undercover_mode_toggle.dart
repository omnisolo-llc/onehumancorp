import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:ui';

final undercoverModeProvider = StateProvider<bool>((ref) => false);

class UndercoverModeToggle extends ConsumerWidget {
  const UndercoverModeToggle({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isUndercover = ref.watch(undercoverModeProvider);

    return ClipRRect(
      borderRadius: BorderRadius.circular(30),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          decoration: BoxDecoration(
            color: isUndercover
                ? Colors.black.withValues(alpha: 0.6)
                : Colors.white.withValues(alpha: 0.1),
            borderRadius: BorderRadius.circular(30),
            border: Border.all(
              color: Colors.white.withValues(alpha: 0.2),
              width: 1,
            ),
          ),
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                isUndercover ? Icons.visibility_off : Icons.visibility,
                color: isUndercover ? Colors.greenAccent : Colors.white,
                size: 20,
              ),
              const SizedBox(width: 12),
              Text(
                'Undercover Mode',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontWeight: FontWeight.bold,
                  fontSize: 14,
                  color: isUndercover ? Colors.greenAccent : Colors.white,
                ),
              ),
              const SizedBox(width: 12),
              Semantics(
                label: 'Undercover Mode Toggle',
                child: Switch(
                  value: isUndercover,
                  onChanged: (value) => ref.read(undercoverModeProvider.notifier).state = value,
                  activeColor: Colors.greenAccent,
                  activeTrackColor: Colors.greenAccent.withValues(alpha: 0.3),
                  inactiveThumbColor: Colors.white70,
                  inactiveTrackColor: Colors.white24,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
