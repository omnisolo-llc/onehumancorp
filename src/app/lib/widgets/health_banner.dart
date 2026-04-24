import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/health_service.dart';

class HealthBanner extends ConsumerWidget {
  const HealthBanner({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final healthState = ref.watch(healthProvider);

    return healthState.when(
      data: (status) {
        if (status == HealthStatus.healthy) {
          return const SizedBox.shrink();
        }

        final isDown = status == HealthStatus.down;
        final color = isDown ? Colors.red.shade800 : Colors.orange.shade800;
        final icon = isDown ? Icons.cloud_off : Icons.warning_amber_rounded;
        final message = isDown
            ? 'Backend Unreachable — Dashboard operating in offline mode'
            : 'Backend Unhealthy — System performance may be degraded';

        return ClipRect(
          child: BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              decoration: BoxDecoration(
                color: color.withValues(alpha: 0.15),
                border: Border(
                  bottom: BorderSide(
                    color: color.withValues(alpha: 0.3),
                    width: 1,
                  ),
                ),
              ),
              child: Row(
                children: [
                   Icon(icon, color: color, size: 20),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      message,
                      style: TextStyle(
                        color: color,
                        fontSize: 13,
                        fontWeight: FontWeight.w600,
                        fontFamily: 'Inter',
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        );
      },
      loading: () => const SizedBox.shrink(),
      error: (_, __) => const SizedBox.shrink(),
    );
  }
}
