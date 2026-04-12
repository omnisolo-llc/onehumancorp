import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

final taskListProvider = FutureProvider<List<Task>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) return [];
  return api.listTasks();
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsyncValue = ref.watch(taskListProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit'))),
      body: Container(
        decoration: const BoxDecoration(
          color: Color(0xFF121212),
        ),
        child: tasksAsyncValue.when(
          data: (tasks) {
            if (tasks.isEmpty) {
              return const Center(child: Text('No tasks found.', style: TextStyle(fontFamily: 'Outfit', color: Colors.white)));
            }
            return ListView.builder(
              padding: const EdgeInsets.all(16),
              itemCount: tasks.length,
              itemBuilder: (context, index) {
                return Padding(
                  padding: const EdgeInsets.only(bottom: 16.0),
                  child: TaskGlassCard(task: tasks[index]),
                );
              },
            );
          },
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (err, stack) => Center(child: Text('Error: $err', style: const TextStyle(fontFamily: 'Outfit', color: Colors.red))),
        ),
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final Task task;
  const TaskGlassCard({super.key, required this.task});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      child: Material(
        type: MaterialType.transparency,
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
              style: const TextStyle(fontFamily: 'Outfit', color: Colors.white70),
            ),
            if (task.assignedAgent != null) ...[
              const SizedBox(height: 4),
              Text(
                'Agent: ${task.assignedAgent}',
                style: const TextStyle(fontFamily: 'Outfit', color: Colors.white70),
              ),
            ],
            if (task.dependencies.isNotEmpty) ...[
              const SizedBox(height: 4),
              Text(
                'Dependencies: ${task.dependencies.join(", ")}',
                style: const TextStyle(fontFamily: 'Outfit', color: Colors.white70),
              ),
            ]
          ],
        ),
      ),
    );
  }
}
