import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final undercoverModeProvider = StateProvider<bool>((ref) => false);

class UndercoverModeToggle extends ConsumerStatefulWidget {
  const UndercoverModeToggle({super.key});

  @override
  ConsumerState<UndercoverModeToggle> createState() => _UndercoverModeToggleState();
}

class _UndercoverModeToggleState extends ConsumerState<UndercoverModeToggle> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _scaleAnimation;
  late Animation<Color?> _colorAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );
    _scaleAnimation = Tween<double>(begin: 1.0, end: 1.1).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _colorAnimation = ColorTween(
      begin: Colors.grey.withValues(alpha: 0.5),
      end: Theme.of(context).colorScheme.primary,
    ).animate(CurvedAnimation(parent: _controller, curve: Curves.easeInOut));
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final isUndercover = ref.watch(undercoverModeProvider);

    if (isUndercover) {
      _controller.forward();
    } else {
      _controller.reverse();
    }

    return Semantics(
      label: 'Undercover Mode Toggle',
      child: GestureDetector(
        onTap: () {
          ref.read(undercoverModeProvider.notifier).state = !isUndercover;
        },
        child: ClipRRect(
          borderRadius: BorderRadius.circular(20),
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            child: AnimatedBuilder(
              animation: _controller,
              builder: (context, child) {
                return Container(
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                  decoration: BoxDecoration(
                    color: const Color.fromRGBO(255, 255, 255, 0.05),
                    borderRadius: BorderRadius.circular(20),
                    border: Border.all(color: _colorAnimation.value ?? Colors.white.withValues(alpha: 0.1)),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      ScaleTransition(
                        scale: _scaleAnimation,
                        child: Icon(
                          isUndercover ? Icons.visibility_off : Icons.visibility,
                          color: _colorAnimation.value,
                          size: 20,
                        ),
                      ),
                      const SizedBox(width: 8),
                      Text(
                        'Undercover Mode',
                        style: TextStyle(
                          fontFamily: 'Inter',
                          fontWeight: FontWeight.bold,
                          color: _colorAnimation.value,
                        ),
                      ),
                    ],
                  ),
                );
              },
            ),
          ),
        ),
      ),
    );
  }
}
