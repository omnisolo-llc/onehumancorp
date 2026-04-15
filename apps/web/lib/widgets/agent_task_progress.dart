import 'dart:ui';
import 'package:flutter/material.dart';

enum TaskStatus { inProgress, completed, failed }

class AgentTaskProgressWidget extends StatelessWidget {
  final String taskName;
  final double progress;
  final TaskStatus status;

  const AgentTaskProgressWidget({
    Key? key,
    required this.taskName,
    required this.progress,
    this.status = TaskStatus.inProgress,
  }) : super(key: key);

  Color get _statusColor {
    switch (status) {
      case TaskStatus.completed:
        return Colors.greenAccent;
      case TaskStatus.failed:
        return Colors.redAccent;
      case TaskStatus.inProgress:
      default:
        return Colors.blueAccent;
    }
  }

  IconData get _statusIcon {
    switch (status) {
      case TaskStatus.completed:
        return Icons.check_circle_outline;
      case TaskStatus.failed:
        return Icons.error_outline;
      case TaskStatus.inProgress:
      default:
        return Icons.hourglass_empty;
    }
  }

  String get _statusText {
    switch (status) {
      case TaskStatus.completed:
        return 'Completed';
      case TaskStatus.failed:
        return 'Failed';
      case TaskStatus.inProgress:
      default:
        return 'In Progress';
    }
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(16.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    taskName,
                    style: const TextStyle(
                      color: Colors.white,
                      fontFamily: 'Outfit',
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  Row(
                    children: [
                      Icon(_statusIcon, color: _statusColor, size: 16),
                      const SizedBox(width: 4),
                      Text(
                        _statusText,
                        style: TextStyle(
                          color: _statusColor,
                          fontFamily: 'Outfit',
                          fontSize: 12,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 12),
              ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: LinearProgressIndicator(
                  value: progress,
                  backgroundColor: const Color.fromRGBO(255, 255, 255, 0.1),
                  valueColor: AlwaysStoppedAnimation<Color>(_statusColor),
                  minHeight: 8,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
