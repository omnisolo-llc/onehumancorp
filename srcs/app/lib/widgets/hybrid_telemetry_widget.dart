import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/screens/dashboard_screen.dart';

class HybridTelemetryWidget extends ConsumerWidget {
  const HybridTelemetryWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final snapshot = ref.watch(dashboardProvider);

    return snapshot.when(
      loading: () => const Center(child: CircularProgressIndicator()),
      error: (err, stack) => Center(child: Text('Error: $err', style: const TextStyle(color: Colors.white))),
      data: (data) {
        // Calculate total tasks dynamically
        int totalTasks = 0;
        for (var status in data.statuses) {
          totalTasks += status.count;
        }

        // Calculate the split
        int cloudTasks = 0;
        int standaloneTasks = 0;
        if (data.hybridHealth != null && data.hybridHealth!.mode == 'cloud_native') {
           cloudTasks = totalTasks;
        } else if (data.hybridHealth != null && data.hybridHealth!.mode == 'standalone') {
           standaloneTasks = totalTasks;
        } else {
           cloudTasks = totalTasks ~/ 2;
           standaloneTasks = totalTasks - cloudTasks;
        }

        return ClipRRect(
          borderRadius: BorderRadius.circular(16),
          child: BackdropFilter(
            filter: ImageFilter.compose(
              outer: const ColorFilter.matrix(<double>[
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
                color: Colors.white.withOpacity(0.03),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: Colors.white.withOpacity(0.1),
                  width: 1,
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text(
                    'Hybrid Deployment Telemetry',
                    style: TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                      fontFamily: 'Outfit',
                    ),
                  ),
                  const SizedBox(height: 16),
                  SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: Row(
                      children: [
                        _TelemetryStat(
                          label: 'Cloud-Native Throughput',
                          value: '$cloudTasks tasks',
                          color: Colors.blueAccent,
                          icon: Icons.cloud,
                        ),
                        const SizedBox(width: 16),
                        _TelemetryStat(
                          label: 'Standalone Throughput',
                          value: '$standaloneTasks tasks',
                          color: Colors.greenAccent,
                          icon: Icons.computer,
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}

class _TelemetryStat extends StatelessWidget {
  final String label;
  final String value;
  final Color color;
  final IconData icon;

  const _TelemetryStat({
    required this.label,
    required this.value,
    required this.color,
    required this.icon,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 250,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withOpacity(0.3)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(icon, color: color, size: 24),
          const SizedBox(height: 8),
          Text(
            value,
            style: TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: color,
              fontFamily: 'Inter',
            ),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          const SizedBox(height: 4),
          Text(
            label,
            style: const TextStyle(
              fontSize: 14,
              color: Colors.white70,
              fontFamily: 'Inter',
            ),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      ),
    );
  }
}
