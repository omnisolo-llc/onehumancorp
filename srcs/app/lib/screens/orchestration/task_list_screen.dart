import 'package:flutter/material.dart';
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/models/orchestration/shared_task.dart';

class TaskListScreen extends StatelessWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final mockTasks = [
      SharedTask(
        id: '1',
        title: 'Implement Shared Task List',
        description: 'Build the UI for the shared task list.',
        status: 'IN_PROGRESS',
        assignedAgent: 'Palette',
        dependencies: [],
      ),
    ];

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          color: Color(0xFF1E1E2E),
        ),
        child: ListView.builder(
          padding: const EdgeInsets.all(24),
          itemCount: mockTasks.length,
          itemBuilder: (context, index) {
            return TaskGlassCard(task: mockTasks[index]);
          },
        ),
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final SharedTask task;

  const TaskGlassCard({super.key, required this.task});

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
            'Status: ${task.status} | Agent: ${task.assignedAgent ?? 'Unassigned'}',
            style: const TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: Colors.white70,
            ),
          ),
        ],
      ),
    );
  }
}
