import 'dart:ui';
import 'package:flutter/material.dart';

class ObservabilityWidget extends StatelessWidget {
  final bool isConnected;
  final int activeMissions;
  final int latencyMs;
  final double cpuUsage;
  final double memoryUsage;

  const ObservabilityWidget({
    super.key,
    required this.isConnected,
    required this.activeMissions,
    required this.latencyMs,
    required this.cpuUsage,
    required this.memoryUsage,
  });

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    return Semantics(
      label: 'System Health and Observability Metrics',
      child: Tooltip(
        message: 'Internal Analytics Dashboard',
        child: ClipRRect(
          borderRadius: BorderRadius.circular(20),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: ColorFilter.matrix(<double>[
                1.168, -0.153, -0.015, 0, 0,
                -0.046, 1.061, -0.015, 0, 0,
                -0.046, -0.152, 1.198, 0, 0,
                0, 0, 0, 1, 0,
              ]),
              inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
            ),
            child: Container(
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: colors.surface.withValues(alpha: 0.1),
                borderRadius: BorderRadius.circular(20),
                border: Border.all(
                  color: colors.outlineVariant.withValues(alpha: 0.2),
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.monitor_heart_outlined,
                        color: colors.primary,
                        size: 28,
                      ),
                      const SizedBox(width: 12),
                      const Text(
                        'System Health',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Outfit',
                        ),
                      ),
                      const Spacer(),
                      _StatusBadge(isConnected: isConnected),
                    ],
                  ),
                  const SizedBox(height: 24),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      _MetricItem(
                        icon: Icons.speed,
                        label: 'Latency',
                        value: '${latencyMs}ms',
                        color: latencyMs < 100 ? Colors.green : (latencyMs < 300 ? Colors.orange : Colors.red),
                      ),
                      _MetricItem(
                        icon: Icons.rocket_launch,
                        label: 'Missions',
                        value: '$activeMissions',
                        color: colors.secondary,
                      ),
                      _MetricItem(
                        icon: Icons.memory,
                        label: 'Compute',
                        value: '${(cpuUsage * 100).toStringAsFixed(1)}%',
                        color: colors.tertiary,
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _StatusBadge extends StatelessWidget {
  final bool isConnected;

  const _StatusBadge({required this.isConnected});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: (isConnected ? Colors.green : Colors.red).withValues(alpha: 0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: (isConnected ? Colors.green : Colors.red).withValues(alpha: 0.5),
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 8,
            height: 8,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: isConnected ? Colors.green : Colors.red,
            ),
          ),
          const SizedBox(width: 6),
          Text(
            isConnected ? 'ONLINE' : 'OFFLINE',
            style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.bold,
              color: isConnected ? Colors.green : Colors.red,
              fontFamily: 'Inter',
            ),
          ),
        ],
      ),
    );
  }
}

class _MetricItem extends StatelessWidget {
  final IconData icon;
  final String label;
  final String value;
  final Color color;

  const _MetricItem({
    required this.icon,
    required this.label,
    required this.value,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Icon(icon, color: color, size: 24),
        const SizedBox(height: 8),
        Text(
          value,
          style: TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.bold,
            color: Theme.of(context).colorScheme.onSurface,
            fontFamily: 'Inter',
          ),
        ),
        const SizedBox(height: 4),
        Text(
          label,
          style: TextStyle(
            fontSize: 12,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
            fontFamily: 'Inter',
          ),
        ),
      ],
    );
  }
}
