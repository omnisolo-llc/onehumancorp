import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/models/task_model.dart';

final tasksProvider = FutureProvider<List<TaskModel>>((ref) async {
  // Simulating API fetch
  await Future.delayed(const Duration(seconds: 1));
  return [
    TaskModel(
      id: '1',
      title: 'Analyze market data',
      assignedAgent: 'Agent Alpha',
      status: 'IN_PROGRESS',
      dependencies: [],
    ),
    TaskModel(
      id: '2',
      title: 'Generate UI components',
      assignedAgent: 'Agent Palette',
      status: 'PENDING',
      dependencies: ['1'],
    ),
    TaskModel(
      id: '3',
      title: 'Deploy to Cloud',
      assignedAgent: 'Agent Maintainer',
      status: 'REVIEW',
      dependencies: ['2'],
    ),
  ];
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsyncValue = ref.watch(tasksProvider);

    return Scaffold(
      backgroundColor: Colors.black, // Assuming dark theme baseline
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF1E1E2C), Color(0xFF232336)],
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Shared Task List',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 32,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 24),
              Expanded(
                child: tasksAsyncValue.when(
                  data: (tasks) => ListView.builder(
                    itemCount: tasks.length,
                    itemBuilder: (context, index) {
                      return TaskGlassCard(task: tasks[index]);
                    },
                  ),
                  loading: () => const Center(child: CircularProgressIndicator(color: Colors.white)),
                  error: (err, stack) => Center(child: Text('Error: $err', style: const TextStyle(color: Colors.redAccent))),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final TaskModel task;

  const TaskGlassCard({super.key, required this.task});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16.0),
      child: GlassCard(
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
                      fontSize: 20,
                      fontWeight: FontWeight.w600,
                      color: Colors.white,
                    ),
                  ),
                ),
                _buildStatusChip(task.status),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              'Agent: ${task.assignedAgent}',
              style: TextStyle(
                fontFamily: 'Inter',
                fontSize: 14,
                color: Colors.white.withValues(alpha: 0.8),
              ),
            ),
            const SizedBox(height: 8),
            if (task.dependencies.isNotEmpty)
              Text(
                'Dependencies: ${task.dependencies.join(", ")}',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 12,
                  color: Colors.white.withValues(alpha: 0.5),
                ),
              ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatusChip(String status) {
    Color color;
    switch (status) {
      case 'PENDING':
        color = Colors.orangeAccent;
        break;
      case 'IN_PROGRESS':
        color = Colors.blueAccent;
        break;
      case 'REVIEW':
        color = Colors.purpleAccent;
        break;
      case 'COMPLETED':
        color = Colors.greenAccent;
        break;
      case 'FAILED':
        color = Colors.redAccent;
        break;
      default:
        color = Colors.grey;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withValues(alpha: 0.5)),
      ),
      child: Text(
        status,
        style: TextStyle(
          fontFamily: 'Inter',
          fontSize: 12,
          fontWeight: FontWeight.w600,
          color: color,
        ),
      ),
    );
  }
}
