import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/shared_task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

final sharedTasksProvider = FutureProvider.autoDispose<List<SharedTask>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getSharedTasks();
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksSnapshot = ref.watch(sharedTasksProvider);

    return Scaffold(
      backgroundColor: Colors.black, // Dark background to make glassmorphism visible
      appBar: AppBar(
        title: const Text(
          'Shared Task List',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: tasksSnapshot.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (err, stack) => Center(child: Text('Error: $err', style: const TextStyle(color: Colors.red, fontFamily: 'Outfit'))),
        data: (tasks) {
          if (tasks.isEmpty) {
            return const Center(child: Text('No tasks available', style: TextStyle(color: Colors.white, fontFamily: 'Outfit')));
          }
          return Padding(
            padding: const EdgeInsets.all(16.0),
            child: ListView.builder(
              itemCount: tasks.length,
              itemBuilder: (context, index) {
                final task = tasks[index];
                return TaskGlassCard(task: task);
              },
            ),
          );
        },
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final SharedTask task;

  const TaskGlassCard({super.key, required this.task});

  Color _getStatusColor(String status) {
    switch (status.toUpperCase()) {
      case 'PENDING':
        return Colors.orangeAccent;
      case 'ASSIGNED':
        return Colors.blueAccent;
      case 'IN_PROGRESS':
        return Colors.cyanAccent;
      case 'REVIEW':
        return Colors.purpleAccent;
      case 'COMPLETED':
        return Colors.greenAccent;
      case 'FAILED':
        return Colors.redAccent;
      default:
        return Colors.grey;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16.0),
      child: GlassCard(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Text(
                    task.title,
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                ),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
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
                      color: _getStatusColor(task.status),
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                const Icon(Icons.support_agent, color: Colors.white70, size: 16),
                const SizedBox(width: 8),
                Text(
                  task.assignedAgent ?? 'Unassigned',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: task.assignedAgent != null ? Colors.white : Colors.white54,
                    fontStyle: task.assignedAgent != null ? FontStyle.normal : FontStyle.italic,
                  ),
                ),
              ],
            ),
            if (task.dependencies.isNotEmpty) ...[
              const SizedBox(height: 12),
              Text(
                'Dependencies: ${task.dependencies.join(", ")}',
                style: const TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 12,
                  color: Colors.white54,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
