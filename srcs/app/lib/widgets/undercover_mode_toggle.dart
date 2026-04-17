import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final undercoverModeProvider = StateProvider<bool>((ref) => false);

class UndercoverModeToggle extends ConsumerWidget {
  const UndercoverModeToggle({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isUndercover = ref.watch(undercoverModeProvider);

    return Semantics(
      label: 'Undercover Mode Toggle',
      toggled: isUndercover,
      child: GestureDetector(
        onTap: () => ref.read(undercoverModeProvider.notifier).state = !isUndercover,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(20),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 300),
              width: 60,
              height: 32,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(20),
                color: isUndercover
                    ? Colors.deepPurpleAccent.withValues(alpha: 0.8)
                    : Colors.white.withValues(alpha: 0.1),
                border: Border.all(
                  color: isUndercover
                      ? Colors.deepPurpleAccent
                      : Colors.white.withValues(alpha: 0.2),
                  width: 1.5,
                ),
              ),
              child: AnimatedAlign(
                duration: const Duration(milliseconds: 300),
                curve: Curves.easeInOut,
                alignment: isUndercover ? Alignment.centerRight : Alignment.centerLeft,
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 4.0),
                  child: Container(
                    width: 24,
                    height: 24,
                    decoration: const BoxDecoration(
                      shape: BoxShape.circle,
                      color: Colors.white,
                    ),
                    child: Icon(
                      isUndercover ? Icons.visibility_off : Icons.visibility,
                      size: 16,
                      color: isUndercover ? Colors.deepPurpleAccent : Colors.grey,
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
