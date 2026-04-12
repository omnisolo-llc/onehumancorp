import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';

// Dummy task model for UI construction until API is integrated
class SharedTask {
  final String id;
  final String title;
  final String assignedAgent;
  final String status;
  final List<String> dependencies;

  SharedTask({
    required this.id,
    required this.title,
    required this.assignedAgent,
    required this.status,
    this.dependencies = const [],
  });
}

// Dummy provider for UI construction
final sharedTasksProvider = Provider<List<SharedTask>>((ref) {
  return [
    SharedTask(
      id: 'task-1',
      title: 'Analyze User Feedback',
      assignedAgent: 'Research Agent',
      status: 'COMPLETED',
    ),
    SharedTask(
      id: 'task-2',
      title: 'Draft Feature Specification',
      assignedAgent: 'Architect Agent',
      status: 'IN_PROGRESS',
      dependencies: ['task-1'],
    ),
    SharedTask(
      id: 'task-3',
      title: 'Implement UI Component',
      assignedAgent: 'Palette Agent',
      status: 'PENDING',
      dependencies: ['task-2'],
    ),
  ];
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasks = ref.watch(sharedTasksProvider);

    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        title: const Text(
          'Shared Task List',
          style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold),
        ),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: Container(
        decoration: const BoxDecoration(
          color: Colors.transparent, // Requires parent to have background if needed
        ),
        child: ListView.builder(
          padding: const EdgeInsets.all(16.0),
          itemCount: tasks.length,
          itemBuilder: (context, index) {
            final task = tasks[index];
            return Padding(
              padding: const EdgeInsets.only(bottom: 16.0),
              child: TaskGlassCard(task: task),
            );
          },
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
      case 'COMPLETED':
        return Colors.greenAccent;
      case 'IN_PROGRESS':
        return Colors.blueAccent;
      case 'PENDING':
        return Colors.orangeAccent;
      case 'FAILED':
        return Colors.redAccent;
      default:
        return Colors.white;
    }
  }

  @override
  Widget build(BuildContext context) {
    return GlassCard(
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
                    color: _getStatusColor(task.status).withOpacity(0.2),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(
                      color: _getStatusColor(task.status).withOpacity(0.5),
                    ),
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
            const SizedBox(height: 12),
            Row(
              children: [
                const Icon(Icons.psychology, color: Colors.white70, size: 16),
                const SizedBox(width: 8),
                Text(
                  task.assignedAgent,
                  style: const TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 14,
                    color: Colors.white70,
                  ),
                ),
              ],
            ),
            if (task.dependencies.isNotEmpty) ...[
              const SizedBox(height: 12),
              Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Icon(Icons.account_tree, color: Colors.white54, size: 16),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      'Dependencies: ${task.dependencies.join(', ')}',
                      style: const TextStyle(
                        fontFamily: 'Inter',
                        fontSize: 12,
                        color: Colors.white54,
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ],
        ),
      ),
    );
  }
}
