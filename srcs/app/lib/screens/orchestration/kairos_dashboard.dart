import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/widgets/pulse_animation.dart';
import 'package:ohc_app/services/centrifuge_service.dart';
import 'package:ohc_app/screens/orchestration/task_list_screen.dart';

class KairosDashboardScreen extends ConsumerWidget {
  const KairosDashboardScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsync = ref.watch(sharedTasksProvider);

    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        title: const Text(
          'KAIROS Orchestration',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        backgroundColor: Colors.transparent,
      ),
      body: ListView(
        padding: const EdgeInsets.all(24),
        children: [
          Text(
            'Live Task Topology',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: Theme.of(context).colorScheme.onSurface,
            ),
          ),
          const SizedBox(height: 16),
          tasksAsync.when(
            data: (tasks) {
              if (tasks.isEmpty) {
                return GlassCard(
                  child: Padding(
                    padding: const EdgeInsets.all(24.0),
                    child: Center(
                      child: Text(
                        'No active missions in queue',
                        style: TextStyle(color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.5), fontFamily: 'Inter'),
                      ),
                    ),
                  ),
                );
              }
              return Column(
                children: tasks.map((task) {
                  final card = GlassCard(
                    margin: const EdgeInsets.only(bottom: 16),
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Expanded(
                                child: Text(
                                  task.title,
                                  style: TextStyle(
                                    fontFamily: 'Outfit',
                                    fontSize: 18,
                                    fontWeight: FontWeight.bold,
                                    color: Theme.of(context).colorScheme.onSurface,
                                  ),
                                ),
                              ),
                              Container(
                                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                                decoration: BoxDecoration(
                                  color: _getStatusColor(task.status).withValues(alpha: 0.2),
                                  borderRadius: BorderRadius.circular(12),
                                  border: Border.all(color: _getStatusColor(task.status).withValues(alpha: 0.5)),
                                ),
                                child: Text(
                                  task.status,
                                  style: TextStyle(
                                    fontFamily: 'Inter',
                                    fontSize: 12,
                                    fontWeight: FontWeight.bold,
                                    color: _getStatusColor(task.status),
                                  ),
                                ),
                              ),
                            ],
                          ),
                          if (task.agentId != null) ...[
                            const SizedBox(height: 8),
                            Row(
                              children: [
                                Icon(Icons.smart_toy, size: 16, color: Theme.of(context).colorScheme.onSurfaceVariant),
                                const SizedBox(width: 8),
                                Text(
                                  'Assigned to: ${task.agentId}',
                                  style: TextStyle(fontFamily: 'Inter', color: Theme.of(context).colorScheme.onSurfaceVariant),
                                ),
                              ],
                            ),
                          ],
                        ],
                      ),
                    ),
                  );

                  if (task.status.toUpperCase() == 'IN_PROGRESS') {
                    return PulseAnimation(child: card);
                  }
                  return card;
                }).toList(),
              );
            },
            loading: () => const Center(child: CircularProgressIndicator()),
            error: (err, stack) => Center(child: Text('Error: $err', style: TextStyle(color: Theme.of(context).colorScheme.error))),
          ),
          const SizedBox(height: 32),
          Text(
            'Teammate Mesh (Live Events)',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: Theme.of(context).colorScheme.onSurface,
            ),
          ),
          const SizedBox(height: 16),
          const MeshEventLog(),
        ],
      ),
    );
  }

  Color _getStatusColor(String status) {
    switch (status.toUpperCase()) {
      case 'COMPLETED':
        return Colors.greenAccent;
      case 'IN_PROGRESS':
        return Colors.blueAccent;
      case 'FAILED':
        return Colors.redAccent;
      case 'REVIEW':
        return Colors.orangeAccent;
      case 'ASSIGNED':
        return Colors.purpleAccent;
      default:
        return Colors.grey;
    }
  }
}

class MeshEventLog extends ConsumerWidget {
  const MeshEventLog({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final centrifuge = ref.watch(centrifugeServiceProvider);

    return GlassCard(
      child: Container(
        height: 200,
        padding: const EdgeInsets.all(24.0),
        child: Builder(
          builder: (context) {
            if (centrifuge == null) {
              return _buildEmptyState(context);
            }
            return StreamBuilder<dynamic>(
              stream: centrifuge.subscribeRaw('mesh:tasks'),
              builder: (context, snapshot) {
                if (!snapshot.hasData) {
                  return _buildEmptyState(context);
                }

                return Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Icon(Icons.auto_awesome, color: Theme.of(context).colorScheme.primary, size: 24),
                        const SizedBox(width: 8),
                        const Text(
                          'Latest Task Event',
                          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold, fontSize: 18),
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    Expanded(
                      child: SingleChildScrollView(
                        child: Text(
                          snapshot.data.toString(),
                          style: TextStyle(fontFamily: 'Inter', color: Theme.of(context).colorScheme.onSurface),
                        ),
                      ),
                    ),
                  ],
                );
              },
            );
          },
        ),
      ),
    );
  }

  Widget _buildEmptyState(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Icon(Icons.wifi_tethering, size: 48, color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.2)),
        const SizedBox(height: 16),
        Text(
          'Listening to mesh:tasks...',
          style: TextStyle(
            fontFamily: 'Inter',
            color: Theme.of(context).colorScheme.onSurface.withValues(alpha: 0.5),
            fontStyle: FontStyle.italic,
          ),
        ),
      ],
    );
  }
}
