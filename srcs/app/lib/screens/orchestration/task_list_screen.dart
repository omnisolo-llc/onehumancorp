import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:ohc_app/widgets/glass_card.dart';
import 'package:ohc_app/models/orchestration/task.dart';

class TaskListScreen extends StatefulWidget {
  const TaskListScreen({super.key});

  @override
  State<TaskListScreen> createState() => _TaskListScreenState();
}

class _TaskListScreenState extends State<TaskListScreen> {
  List<SwarmTask> _tasks = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _fetchTasks();
  }

  Future<void> _fetchTasks() async {
    try {
      const baseUrl = String.fromEnvironment('API_BASE_URL', defaultValue: 'http://localhost:8080/api/v1');

      final response = await http.get(Uri.parse('$baseUrl/tasks')).timeout(const Duration(seconds: 1));

      if (response.statusCode == 200) {
        final List<dynamic> data = json.decode(response.body);
        setState(() {
          _tasks = data.map((json) => SwarmTask.fromJson(json)).toList();
          _isLoading = false;
        });
      } else {
        setState(() {
          _error = 'Failed to load tasks: ${response.statusCode}';
          _isLoading = false;
        });
      }
    } catch (e) {
      setState(() {
        // Fallback for testing/design if API is unreachable
        _tasks = [
          SwarmTask(id: '1', title: 'Analyze Market Data', description: 'Gather and analyze current market trends.', status: 'IN_PROGRESS', assignedAgent: 'Agent Alpha', dependencies: []),
          SwarmTask(id: '2', title: 'Generate Report', description: 'Create a comprehensive report based on market analysis.', status: 'PENDING', dependencies: ['1']),
          SwarmTask(id: '3', title: 'Review Report', description: 'Review the generated report for accuracy.', status: 'PENDING', dependencies: ['2']),
        ];
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent, // Allow shell background to show
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _error != null
              ? Center(child: Text(_error!, style: const TextStyle(color: Colors.red)))
              : ListView.builder(
                  padding: const EdgeInsets.all(16),
                  itemCount: _tasks.length,
                  itemBuilder: (context, index) {
                    return TaskGlassCard(task: _tasks[index]);
                  },
                ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final SwarmTask task;

  const TaskGlassCard({super.key, required this.task});

  @override
  Widget build(BuildContext context) {
    return GlassCard(
      margin: const EdgeInsets.only(bottom: 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
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
              _buildStatusBadge(task.status),
            ],
          ),
          const SizedBox(height: 8),
          Text(
            task.description,
            style: TextStyle(
              fontFamily: 'Inter',
              fontSize: 14,
              color: Colors.white.withValues(alpha: 0.8),
            ),
          ),
          const SizedBox(height: 12),
          Row(
            children: [
              Icon(Icons.smart_toy, size: 16, color: Colors.white.withValues(alpha: 0.6)),
              const SizedBox(width: 4),
              Text(
                task.assignedAgent ?? 'Unassigned',
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 12,
                  color: Colors.white.withValues(alpha: 0.6),
                ),
              ),
              const Spacer(),
              if (task.dependencies.isNotEmpty) ...[
                Icon(Icons.account_tree, size: 16, color: Colors.white.withValues(alpha: 0.6)),
                const SizedBox(width: 4),
                Text(
                  '${task.dependencies.length} deps',
                  style: TextStyle(
                    fontFamily: 'Inter',
                    fontSize: 12,
                    color: Colors.white.withValues(alpha: 0.6),
                  ),
                ),
              ]
            ],
          )
        ],
      ),
    );
  }

  Widget _buildStatusBadge(String status) {
    Color badgeColor;
    switch (status.toUpperCase()) {
      case 'COMPLETED':
        badgeColor = Colors.green;
        break;
      case 'IN_PROGRESS':
        badgeColor = Colors.blue;
        break;
      case 'FAILED':
        badgeColor = Colors.red;
        break;
      default:
        badgeColor = Colors.orange;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: badgeColor.withValues(alpha: 0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: badgeColor.withValues(alpha: 0.5)),
      ),
      child: Text(
        status,
        style: TextStyle(
          fontFamily: 'Inter',
          fontSize: 10,
          fontWeight: FontWeight.bold,
          color: badgeColor,
        ),
      ),
    );
  }
}
