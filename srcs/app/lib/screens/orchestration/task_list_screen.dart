import 'dart:ui';
import 'package:flutter/material.dart';

class Task {
  final String id;
  final String title;
  final String assignee;
  final String status;
  final List<String> dependencies;

  Task({
    required this.id,
    required this.title,
    required this.assignee,
    required this.status,
    this.dependencies = const [],
  });
}

class TaskListScreen extends StatefulWidget {
  const TaskListScreen({super.key});

  @override
  State<TaskListScreen> createState() => _TaskListScreenState();
}

class _TaskListScreenState extends State<TaskListScreen> {
  final List<Task> _tasks = [
    Task(id: '1', title: 'Initialize Hub', assignee: 'Architect', status: 'COMPLETED'),
    Task(id: '2', title: 'Design Teammate Mesh', assignee: 'Researcher', status: 'IN_PROGRESS', dependencies: ['1']),
    Task(id: '3', title: 'Implement Redis PubSub', assignee: 'Implementer', status: 'PENDING', dependencies: ['2']),
    Task(id: '4', title: 'Review Code', assignee: 'Maintainer', status: 'PENDING', dependencies: ['3']),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit'))),
      body: Container(
        color: const Color(0xFF1E1E1E),
        child: ListView.builder(
          padding: const EdgeInsets.all(16),
          itemCount: _tasks.length,
          itemBuilder: (context, index) {
            return Padding(
              padding: const EdgeInsets.only(bottom: 16),
              child: TaskGlassCard(task: _tasks[index]),
            );
          },
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
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: Container(
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(color: Colors.white.withOpacity(0.1)),
          ),
          padding: const EdgeInsets.all(16),
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
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text('Assignee: ${task.assignee}', style: const TextStyle(fontFamily: 'Outfit', color: Colors.white70)),
                  Text('Status: ${task.status}', style: const TextStyle(fontFamily: 'Outfit', color: Colors.white70)),
                ],
              ),
              if (task.dependencies.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text('Deps: ${task.dependencies.join(', ')}', style: const TextStyle(fontFamily: 'Outfit', color: Colors.white54, fontSize: 12)),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
