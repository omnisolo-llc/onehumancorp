import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/models/dashboard.dart';

class SubAgentQueueWidget extends StatefulWidget {
  final List<StatusBucket> statuses;

  const SubAgentQueueWidget({super.key, required this.statuses});

  @override
  State<SubAgentQueueWidget> createState() => _SubAgentQueueWidgetState();
}

class _SubAgentQueueWidgetState extends State<SubAgentQueueWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 2),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 0.8, end: 1.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  int _getCountForStatus(String status) {
    if (widget.statuses.isEmpty) return 0;
    try {
      return widget.statuses.firstWhere(
        (b) => b.status.toLowerCase() == status.toLowerCase(),
        orElse: () => const StatusBucket(status: '', count: 0)
      ).count;
    } catch (_) {
      return 0;
    }
  }

  Widget _buildQueueNode(String label, IconData icon, Color color, String count) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        ScaleTransition(
          scale: color == Colors.blueAccent ? _pulseAnimation : const AlwaysStoppedAnimation(1.0),
          child: Container(
            width: 48,
            height: 48,
            decoration: BoxDecoration(
              color: color.withOpacity(0.2),
              shape: BoxShape.circle,
              border: Border.all(color: color.withOpacity(0.5), width: 2),
              boxShadow: [
                BoxShadow(
                  color: color.withOpacity(0.3),
                  blurRadius: 10,
                  spreadRadius: 2,
                )
              ],
            ),
            child: Icon(icon, color: color, size: 24),
          ),
        ),
        const SizedBox(height: 8),
        Text(
          label,
          style: const TextStyle(
            fontFamily: 'Inter',
            fontSize: 10,
            color: Colors.white70,
            fontWeight: FontWeight.w500,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 4),
        Text(
          count,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 14,
            color: color,
            fontWeight: FontWeight.bold,
          ),
          textAlign: TextAlign.center,
        )
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final enqueued = _getCountForStatus('pending');
    final processing = _getCountForStatus('in_progress');
    final completed = _getCountForStatus('completed');
    final failed = _getCountForStatus('failed');

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(24.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.1)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'Sub-Agent Orchestration Queue',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 24),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _buildQueueNode('Enqueued', Icons.queue, Colors.orangeAccent, enqueued.toString()),
                  const Icon(Icons.arrow_forward, color: Colors.white24),
                  _buildQueueNode('Processing', Icons.autorenew, Colors.blueAccent, processing.toString()),
                  const Icon(Icons.arrow_forward, color: Colors.white24),
                  _buildQueueNode('Completed', Icons.check_circle, Colors.greenAccent, completed.toString()),
                  const SizedBox(width: 8),
                  _buildQueueNode('Failed', Icons.error, Colors.redAccent, failed.toString()),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
