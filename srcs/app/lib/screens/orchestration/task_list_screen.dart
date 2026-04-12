import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ohc_app/models/task.dart';
import 'package:ohc_app/services/api_service.dart';
import 'package:ohc_app/widgets/glass_card.dart';

final taskListProvider = FutureProvider.autoDispose<List<Task>>((ref) async {
  final api = ref.watch(apiServiceProvider);
  if (api == null) throw Exception('API not available');
  return api.getTasks();
});

class TaskListScreen extends ConsumerWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final snapshot = ref.watch(taskListProvider);
    return Scaffold(
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: snapshot.when(
        loading: () => Center(child: CircularProgressIndicator(color: Theme.of(context).colorScheme.primary)),
        error: (err, stack) => Center(child: Text('Error: $err', style: const TextStyle(fontFamily: 'Inter'))),
        data: (tasks) {
          if (tasks.isEmpty) {
            return const Center(child: Text('No tasks available.', style: TextStyle(fontFamily: 'Inter')));
          }
          return ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: tasks.length,
            itemBuilder: (context, index) {
              final task = tasks[index];
              return Padding(
                padding: const EdgeInsets.only(bottom: 16.0),
                child: GlassCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(task.title, style: const TextStyle(fontFamily: 'Outfit', fontSize: 18, fontWeight: FontWeight.bold)),
                      const SizedBox(height: 8),
                      Text('Status: ${task.status}', style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
                      if (task.assignedAgentId != null) Text('Assigned: ${task.assignedAgentId}', style: const TextStyle(fontFamily: 'Inter', fontSize: 14)),
                    ],
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
