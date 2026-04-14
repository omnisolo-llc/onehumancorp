import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final sharedTasksProvider = StateProvider<List<Map<String, dynamic>>>((ref) => []);

class SharedTaskListWidget extends ConsumerWidget {
  const SharedTaskListWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasks = ref.watch(sharedTasksProvider);

    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
      ),
      body: tasks.isEmpty ? const Center(child: Text('No tasks available', style: TextStyle(color: Colors.white))) : ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: tasks.length,
        itemBuilder: (context, index) => _TaskGlassCard(task: tasks[index]),
      ),
    );
  }
}

class _TaskGlassCard extends StatelessWidget {
  final Map<String, dynamic> task;

  const _TaskGlassCard({required this.task});

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.compose(
          outer: ColorFilter.matrix(const <double>[
            1.168, -0.153, -0.015, 0, 0,
            -0.046, 1.061, -0.015, 0, 0,
            -0.046, -0.152, 1.198, 0, 0,
            0, 0, 0, 1, 0,
          ]),
          inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        ),
        child: Container(
          margin: const EdgeInsets.only(bottom: 16),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.05),
            borderRadius: BorderRadius.circular(16),
          ),
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                task['title'] ?? '',
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                'Status: ${task['status'] ?? ''}',
                style: TextStyle(
                  fontFamily: 'Inter',
                  color: _getStatusColor(task['status'] ?? ''),
                ),
              ),
              if (task['agent_id'] != null && task['agent_id'] != '') ...[
                const SizedBox(height: 4),
                Text(
                  'Agent: ${task['agent_id']}',
                  style: const TextStyle(fontFamily: 'Inter', color: Colors.white70),
                ),
              ],
            ],
          ),
        ),
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
