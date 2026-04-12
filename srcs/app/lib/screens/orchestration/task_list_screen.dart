import 'dart:ui';
import 'package:flutter/material.dart';

class TaskListScreen extends StatefulWidget {
  const TaskListScreen({super.key});

  @override
  State<TaskListScreen> createState() => _TaskListScreenState();
}

class _TaskListScreenState extends State<TaskListScreen> {
  final List<Map<String, dynamic>> _tasks = [
    {
      'title': 'Train Nova LLM',
      'agent': 'Nova',
      'status': 'IN_PROGRESS',
      'dependencies': ['Data Gather'],
    },
    {
      'title': 'Deploy Kubernetes Cluster',
      'agent': 'Implementer',
      'status': 'PENDING',
      'dependencies': [],
    },
    {
      'title': 'Review PR',
      'agent': 'Architect',
      'status': 'REVIEW',
      'dependencies': ['Code Implementation'],
    },
    {
      'title': 'Design AutoDream',
      'agent': 'Palette',
      'status': 'COMPLETED',
      'dependencies': [],
    },
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit')),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      backgroundColor: Colors.black87,
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Swarm Tasks',
              style: TextStyle(
                fontSize: 28,
                fontWeight: FontWeight.bold,
                fontFamily: 'Outfit',
                color: Colors.white,
              ),
            ),
            const SizedBox(height: 24),
            Expanded(
              child: ListView.builder(
                itemCount: _tasks.length,
                itemBuilder: (context, index) {
                  final task = _tasks[index];
                  return TaskGlassCard(
                    title: task['title'] as String,
                    agent: task['agent'] as String,
                    status: task['status'] as String,
                    dependencies: List<String>.from(task['dependencies'] as List),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final String title;
  final String agent;
  final String status;
  final List<String> dependencies;

  const TaskGlassCard({
    super.key,
    required this.title,
    required this.agent,
    required this.status,
    required this.dependencies,
  });

  Color _getStatusColor() {
    switch (status) {
      case 'COMPLETED':
        return Colors.green;
      case 'IN_PROGRESS':
        return Colors.blue;
      case 'REVIEW':
        return Colors.orange;
      case 'FAILED':
        return Colors.red;
      case 'PENDING':
      case 'ASSIGNED':
      default:
        return Colors.grey;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.compose(
            outer: const ColorFilter.matrix(<double>[
              1.168, -0.153, -0.015, 0, 0,
              -0.046, 1.061, -0.015, 0, 0,
              -0.046, -0.152, 1.198, 0, 0,
              0, 0, 0, 1, 0,
            ]),
            inner: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          ),
          child: Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: const Color.fromRGBO(255, 255, 255, 0.03),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Expanded(
                      child: Text(
                        title,
                        style: const TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                          color: Colors.white,
                          fontFamily: 'Outfit',
                        ),
                      ),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                      decoration: BoxDecoration(
                        color: _getStatusColor().withValues(alpha: 0.2),
                        borderRadius: BorderRadius.circular(20),
                        border: Border.all(color: _getStatusColor().withValues(alpha: 0.5)),
                      ),
                      child: Text(
                        status,
                        style: TextStyle(
                          color: _getStatusColor(),
                          fontWeight: FontWeight.bold,
                          fontSize: 12,
                          fontFamily: 'Inter',
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                Row(
                  children: [
                    const Icon(Icons.smart_toy, color: Colors.white70, size: 16),
                    const SizedBox(width: 8),
                    Text(
                      agent,
                      style: const TextStyle(color: Colors.white70, fontFamily: 'Inter', fontSize: 14),
                    ),
                  ],
                ),
                if (dependencies.isNotEmpty) ...[
                  const SizedBox(height: 12),
                  Wrap(
                    spacing: 8,
                    children: dependencies.map((dep) {
                      return Container(
                        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                        decoration: BoxDecoration(
                          color: Colors.white.withValues(alpha: 0.05),
                          borderRadius: BorderRadius.circular(8),
                          border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
                        ),
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            const Icon(Icons.link, color: Colors.white54, size: 12),
                            const SizedBox(width: 4),
                            Text(
                              dep,
                              style: const TextStyle(color: Colors.white54, fontSize: 12, fontFamily: 'Inter'),
                            ),
                          ],
                        ),
                      );
                    }).toList(),
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}
