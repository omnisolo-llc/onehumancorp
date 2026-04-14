import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final modeProvider = StateProvider<String>((ref) => 'Cloud Mode');

class ModeIndicator extends ConsumerWidget {
  const ModeIndicator({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final mode = ref.watch(modeProvider);
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(8.0),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 12.0),
            decoration: BoxDecoration(
              color: const Color.fromRGBO(255, 255, 255, 0.03),
              borderRadius: BorderRadius.circular(8.0),
              border: Border.all(color: Colors.white12),
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Icon(
                  mode == 'Cloud Mode' ? Icons.cloud : Icons.computer,
                  color: mode == 'Cloud Mode' ? Colors.blueAccent : Colors.greenAccent,
                  size: 16,
                ),
                const SizedBox(width: 8),
                Text(
                  mode,
                  style: const TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: Colors.white,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
