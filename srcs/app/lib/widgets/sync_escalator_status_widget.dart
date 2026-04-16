import 'dart:ui';
import 'package:flutter/material.dart';

class SyncEscalatorStatusWidget extends StatefulWidget {
  final bool isCloudEscalated;
  final int escalatedTaskCount;

  const SyncEscalatorStatusWidget({
    super.key,
    required this.isCloudEscalated,
    required this.escalatedTaskCount,
  });

  @override
  State<SyncEscalatorStatusWidget> createState() => _SyncEscalatorStatusWidgetState();
}

class _SyncEscalatorStatusWidgetState extends State<SyncEscalatorStatusWidget> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _scaleAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );
    _scaleAnimation = Tween<double>(begin: 0.95, end: 1.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeOutBack),
    );
    _controller.forward();
  }

  @override
  void didUpdateWidget(SyncEscalatorStatusWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.isCloudEscalated != widget.isCloudEscalated) {
      _controller.reset();
      _controller.forward();
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final statusColor = widget.isCloudEscalated ? Colors.blueAccent : Colors.greenAccent;
    final statusText = widget.isCloudEscalated ? 'Cloud Swarm (PostgreSQL)' : 'Local Default (SQLite)';
    final iconData = widget.isCloudEscalated ? Icons.cloud_sync_outlined : Icons.lock_outline;

    return ScaleTransition(
      scale: _scaleAnimation,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(16),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
          child: Container(
            padding: const EdgeInsets.all(20.0),
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
                    const Text(
                      'Hybrid RAG Escalation Status',
                      style: TextStyle(
                        color: Colors.white,
                        fontFamily: 'Outfit',
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    Icon(
                      iconData,
                      color: statusColor,
                      size: 28,
                    ),
                  ],
                ),
                const SizedBox(height: 16),
                Row(
                  children: [
                    Container(
                      width: 12,
                      height: 12,
                      decoration: BoxDecoration(
                        color: statusColor,
                        shape: BoxShape.circle,
                        boxShadow: [
                          BoxShadow(
                            color: statusColor.withOpacity(0.5),
                            blurRadius: 8,
                            spreadRadius: 2,
                          ),
                        ],
                      ),
                    ),
                    const SizedBox(width: 12),
                    Text(
                      statusText,
                      style: TextStyle(
                        color: statusColor,
                        fontFamily: 'Inter',
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                Text(
                  'Escalated Tasks: ${widget.escalatedTaskCount}',
                  style: const TextStyle(
                    color: Colors.white70,
                    fontFamily: 'Inter',
                    fontSize: 14,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
