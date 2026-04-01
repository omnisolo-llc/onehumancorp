import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/services/settings_service.dart';

class ObservabilityWidget extends ConsumerWidget {
  const ObservabilityWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settingsAsync = ref.watch(clientSettingsProvider);

    return settingsAsync.when(
      data: (settings) {
        final isStandalone = settings.standaloneMode;
        final connectionModeStr = isStandalone ? 'Local SQLite' : 'Cloud PostgreSQL';

        return Semantics(
          label: 'Observability Dashboard',
          child: Container(
            width: 350,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(16),
              border: Border.all(
                color: Theme.of(context).colorScheme.outlineVariant.withOpacity(0.3),
                width: 1,
              ),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.05),
                  blurRadius: 10,
                  offset: const Offset(0, 4),
                ),
              ],
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(16),
              child: BackdropFilter(
                filter: ImageFilter.compose(
                  outer: ColorFilter.matrix(<double>[
                    1.168, -0.153, -0.015, 0, 0,
                    -0.046, 1.061, -0.015, 0, 0,
                    -0.046, -0.152, 1.198, 0, 0,
                    0,      0,      0,     1, 0,
                  ]),
                  inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
                ),
                child: Container(
                  color: Theme.of(context).colorScheme.surface.withOpacity(0.1),
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Row(
                        children: [
                          Icon(
                            Icons.monitor_heart,
                            color: Theme.of(context).colorScheme.primary,
                            size: 24,
                          ),
                          const SizedBox(width: 8),
                          Expanded(
                            child: Text(
                              'System Observability',
                              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                                fontWeight: FontWeight.bold,
                                fontFamily: 'Outfit', // Uses generic fallback or defined outfit
                              ),
                              overflow: TextOverflow.ellipsis,
                            ),
                          ),
                          Container(
                            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                            decoration: BoxDecoration(
                              color: Colors.green.withOpacity(0.2),
                              borderRadius: BorderRadius.circular(12),
                            ),
                            child: const Row(
                              children: [
                                Icon(Icons.circle, color: Colors.green, size: 8),
                                SizedBox(width: 4),
                                Text(
                                  'ONLINE',
                                  style: TextStyle(
                                    fontSize: 10,
                                    fontWeight: FontWeight.bold,
                                    color: Colors.green,
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      _MetricRow(
                        icon: Icons.hub,
                        label: 'Connection Mode',
                        value: connectionModeStr,
                        context: context,
                      ),
                      const SizedBox(height: 12),
                      _MetricRow(
                        icon: Icons.speed,
                        label: 'P95 Latency',
                        value: isStandalone ? '< 5ms' : '142ms',
                        context: context,
                        isGood: isStandalone,
                        isWarning: !isStandalone,
                      ),
                      const SizedBox(height: 12),
                      _MetricRow(
                        icon: Icons.memory,
                        label: 'Active Missions',
                        value: '4/10',
                        context: context,
                      ),
                    ],
                  ),
                ),
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

class _MetricRow extends StatelessWidget {
  final IconData icon;
  final String label;
  final String value;
  final BuildContext context;
  final bool isGood;
  final bool isWarning;

  const _MetricRow({
    required this.icon,
    required this.label,
    required this.value,
    required this.context,
    this.isGood = false,
    this.isWarning = false,
  });

  @override
  Widget build(BuildContext context) {
    Color valueColor = Theme.of(context).colorScheme.onSurface;
    if (isGood) valueColor = Colors.green;
    if (isWarning) valueColor = Colors.orange;

    return Row(
      children: [
        Icon(
          icon,
          size: 16,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
        const SizedBox(width: 8),
        Expanded(
          child: Text(
            label,
            style: Theme.of(context).textTheme.bodySmall?.copyWith(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
              fontFamily: 'Inter',
            ),
            overflow: TextOverflow.ellipsis,
          ),
        ),
        const SizedBox(width: 8),
        Text(
          value,
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            fontWeight: FontWeight.bold,
            color: valueColor,
            fontFamily: 'Inter',
          ),
        ),
      ],
    );
  }
}
