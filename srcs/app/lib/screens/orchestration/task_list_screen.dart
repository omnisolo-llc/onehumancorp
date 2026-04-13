import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/models/shared_task.dart';
import 'package:ohc_app/services/api_service.dart';

final sharedTasksProvider = FutureProvider<List<SharedTask>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  final rawTasks = await api.listSharedTasks();
  return rawTasks.map((t) => SharedTask.fromJson(t)).toList();
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsync = ref.watch(sharedTasksProvider);

    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
      ),
      body: tasksAsync.when(
        data: (tasks) => ListView.builder(
          padding: const EdgeInsets.all(16),
          itemCount: tasks.length,
          itemBuilder: (context, index) => _TaskGlassCard(task: tasks[index]),
        ),
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (err, stack) => Center(child: Text('Error: $err', style: TextStyle(color: Colors.white))),
      ),
    );
  }
}

class _TaskGlassCard extends StatelessWidget {
  final SharedTask task;

  const _TaskGlassCard({required this.task});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      margin: const EdgeInsets.only(bottom: 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            task.title,
            style: const TextStyle(
              fontFamily: 'Outfit',
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Status: ${task.status}',
            style: TextStyle(
              fontFamily: 'Inter',
              color: _getStatusColor(task.status),
            ),
          ),
          if (task.agentId != null) ...[
            const SizedBox(height: 4),
            Text(
              'Agent: ${task.agentId}',
              style: const TextStyle(fontFamily: 'Inter', color: Colors.white70),
            ),
          ],
          if (task.dependencies != null && task.dependencies!.isNotEmpty) ...[
            const SizedBox(height: 8),
            Text(
              'Dependencies: ${task.dependencies!.join(', ')}',
              style: const TextStyle(fontFamily: 'Inter', color: Colors.white54, fontSize: 12),
            ),
          ],
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
