import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/swarm_observability_widget.dart';

class SwarmVelocityWidget extends ConsumerStatefulWidget {
  const SwarmVelocityWidget({super.key});

  @override
  ConsumerState<SwarmVelocityWidget> createState() => _SwarmVelocityWidgetState();
}

class _SwarmVelocityWidgetState extends ConsumerState<SwarmVelocityWidget> {
  final List<MeshMessage> _recentMessages = [];
  int _completedTasks = 0;
  double _avgLatencyMs = 0.0;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;

    ref.listen<AsyncValue<MeshMessage>>(meshStreamProvider, (previous, next) {
      if (next.hasValue && next.value != null) {
        setState(() {
          final now = DateTime.now();
          final msg = next.value!;
          _recentMessages.add(msg);

          // Keep messages from the last 60 seconds
          _recentMessages.removeWhere((m) => now.difference(m.timestamp).inSeconds > 60);

          _completedTasks = _recentMessages.where((m) => m.action.toLowerCase().contains('complete') || m.action.toLowerCase().contains('finish') || m.action.toLowerCase().contains('success')).length;

          _avgLatencyMs = 0.0;
        });
      }
    });

    return Semantics(
      label: 'Swarm Velocity Widget',
      child: ClipRRect(
        borderRadius: BorderRadius.circular(24),
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
            padding: const EdgeInsets.all(24.0),
            decoration: BoxDecoration(
              color: const Color.fromRGBO(255, 255, 255, 0.03),
              borderRadius: BorderRadius.circular(24),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Row(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(8),
                      decoration: BoxDecoration(
                        color: colors.secondary.withValues(alpha: 0.2),
                        shape: BoxShape.circle,
                      ),
                      child: Icon(Icons.speed, color: colors.secondary, size: 24),
                    ),
                    const SizedBox(width: 12),
                    const Text(
                      'Swarm Velocity',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 24),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceAround,
                  children: [
                    _VelocityMetric(
                      label: 'Task Completion Rate',
                      value: '$_completedTasks/min',
                      icon: Icons.check_circle_outline,
                      color: Colors.greenAccent,
                    ),
                    _VelocityMetric(
                      label: 'Average Latency',
                      value: '${_avgLatencyMs.toStringAsFixed(0)} ms',
                      icon: Icons.timer_outlined,
                      color: Colors.orangeAccent,
                    ),
                    _VelocityMetric(
                      label: 'Active Threads',
                      value: '${_recentMessages.length}',
                      icon: Icons.account_tree_outlined,
                      color: Colors.cyanAccent,
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _VelocityMetric extends StatelessWidget {
  final String label;
  final String value;
  final IconData icon;
  final Color color;

  const _VelocityMetric({
    required this.label,
    required this.value,
    required this.icon,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Icon(icon, color: color, size: 28),
        const SizedBox(height: 8),
        Text(
          value,
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 24,
            fontWeight: FontWeight.bold,
            color: color,
          ),
        ),
        const SizedBox(height: 4),
        Text(
          label,
          style: TextStyle(
            fontFamily: 'Inter',
            fontSize: 12,
            color: Colors.white.withValues(alpha: 0.7),
          ),
        ),
      ],
    );
  }
}
