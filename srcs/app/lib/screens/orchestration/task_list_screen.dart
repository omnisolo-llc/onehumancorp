import 'dart:ui';
import 'package:flutter/material.dart';

class TaskListScreen extends StatelessWidget {
  const TaskListScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Shared Task List', style: TextStyle(fontFamily: 'Outfit', fontWeight: FontWeight.bold)),
      ),
      body: Container(
        decoration: const BoxDecoration(
          // Ensure a premium background look behind the glass components
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF1E1E2C), Color(0xFF2D2D44)],
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: ListView(
            children: const [
              TaskGlassCard(
                title: 'Implement Core UI',
                agent: 'Palette',
                status: 'IN_PROGRESS',
                dependencies: 'None',
              ),
              SizedBox(height: 16),
              TaskGlassCard(
                title: 'Optimize Database',
                agent: 'Miser',
                status: 'PENDING',
                dependencies: 'Database Setup',
              ),
              SizedBox(height: 16),
              TaskGlassCard(
                title: 'Write Docs',
                agent: 'Scribe',
                status: 'COMPLETED',
                dependencies: 'Core UI',
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class TaskGlassCard extends StatelessWidget {
  final String title;
  final String agent;
  final String status;
  final String dependencies;

  const TaskGlassCard({
    super.key,
    required this.title,
    required this.agent,
    required this.status,
    required this.dependencies,
  });

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.05),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(
              color: Colors.white.withOpacity(0.1),
              width: 1.5,
            ),
          ),
          padding: const EdgeInsets.all(20.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 12),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  _buildLabel('Agent: ', agent),
                  _buildStatusBadge(status),
                ],
              ),
              const SizedBox(height: 8),
              _buildLabel('Dependencies: ', dependencies),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildLabel(String label, String value) {
    return RichText(
      text: TextSpan(
        style: const TextStyle(fontFamily: 'Inter', fontSize: 14, color: Colors.white70),
        children: [
          TextSpan(text: label, style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white)),
          TextSpan(text: value),
        ],
      ),
    );
  }

  Widget _buildStatusBadge(String status) {
    Color statusColor;
    switch (status) {
      case 'IN_PROGRESS':
        statusColor = Colors.blueAccent;
        break;
      case 'COMPLETED':
        statusColor = Colors.greenAccent;
        break;
      case 'FAILED':
        statusColor = Colors.redAccent;
        break;
      case 'REVIEW':
        statusColor = Colors.orangeAccent;
        break;
      case 'ASSIGNED':
        statusColor = Colors.purpleAccent;
        break;
      default:
        statusColor = Colors.grey;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      decoration: BoxDecoration(
        color: statusColor.withOpacity(0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: statusColor.withOpacity(0.5)),
      ),
      child: Text(
        status,
        style: TextStyle(
          fontFamily: 'Inter',
          fontSize: 12,
          fontWeight: FontWeight.bold,
          color: statusColor,
        ),
      ),
    );
  }
}
