import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/models/task.dart';

final taskListProvider = FutureProvider.autoDispose<List<SharedTask>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getTasks();
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final taskSnapshot = ref.watch(taskListProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: Container(
        decoration: const BoxDecoration(
          color: Color(0xFF1E1E1E), // Dark background for contrast with glassmorphism
        ),
        child: taskSnapshot.when(
          data: (tasks) {
            if (tasks.isEmpty) {
              return const Center(
                child: Text(
                  'No tasks available',
                  style: TextStyle(color: Colors.white, fontFamily: 'Inter', fontSize: 16),
                ),
              );
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
          loading: () => Center(
            child: CircularProgressIndicator(
              color: Theme.of(context).colorScheme.primary,
            ),
          ),
          error: (e, _) => Center(
            child: Text(
              'Error loading tasks: $e',
              style: TextStyle(color: Theme.of(context).colorScheme.error, fontFamily: 'Inter'),
            ),
          ),
        ),
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final SharedTask task;

  const TaskGlassCard({super.key, required this.task});

  Color _getStatusColor(String status) {
    switch (status) {
      case 'PENDING':
        return Colors.orangeAccent;
      case 'IN_PROGRESS':
        return Colors.blueAccent;
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
    final statusColor = _getStatusColor(task.status);

    return GlassCard(
      margin: const EdgeInsets.only(bottom: 16.0),
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
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                decoration: BoxDecoration(
                  color: statusColor.withOpacity(0.2),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: statusColor.withOpacity(0.5)),
                ),
                child: Text(
                  task.status,
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 12,
                    fontWeight: FontWeight.w600,
                    color: statusColor,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            'Assigned Agent: ${task.assignedAgent ?? 'Unassigned'}',
            style: const TextStyle(
              fontFamily: 'Inter',
              fontSize: 14,
              color: Colors.white70,
            ),
          ),
          if (task.dependencies.isNotEmpty) ...[
            const SizedBox(height: 8),
            Text(
              'Dependencies: ${task.dependencies.join(', ')}',
              style: const TextStyle(
                fontFamily: 'Inter',
                fontSize: 12,
                color: Colors.white54,
              ),
            ),
          ]
        ],
      ),
    );
  }
}
