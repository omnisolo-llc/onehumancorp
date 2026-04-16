import 'dart:ui';
import 'package:flutter/material.dart';

enum EscalationState { local, escalating, cloud }

class DynamicCloudEscalationWidget extends StatefulWidget {
  final EscalationState state;

  const DynamicCloudEscalationWidget({Key? key, required this.state})
      : super(key: key);

  @override
  State<DynamicCloudEscalationWidget> createState() =>
      _DynamicCloudEscalationWidgetState();
}

class _DynamicCloudEscalationWidgetState
    extends State<DynamicCloudEscalationWidget>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 0.3, end: 1.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Color statusColor;
    String statusText;
    IconData statusIcon;

    switch (widget.state) {
      case EscalationState.local:
        statusColor = Colors.blueAccent;
        statusText = 'Local SQLite (Private)';
        statusIcon = Icons.dns;
        break;
      case EscalationState.escalating:
        statusColor = Colors.orangeAccent;
        statusText = 'Escalating Workload...';
        statusIcon = Icons.cloud_upload;
        break;
      case EscalationState.cloud:
        statusColor = Colors.greenAccent;
        statusText = 'Cloud Swarm (Infinite Scale)';
        statusIcon = Icons.cloud_done;
        break;
    }

    return ClipRRect(
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
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              AnimatedBuilder(
                animation: _pulseAnimation,
                builder: (context, child) {
                  return Container(
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      color: statusColor.withOpacity(_pulseAnimation.value * 0.2),
                      boxShadow: [
                        BoxShadow(
                          color: statusColor.withOpacity(
                            _pulseAnimation.value * 0.4,
                          ),
                          blurRadius: 12.0,
                          spreadRadius: 2.0,
                        ),
                      ],
                      border: Border.all(
                        color: statusColor.withOpacity(_pulseAnimation.value),
                        width: 2,
                      ),
                    ),
                    child: Icon(
                      statusIcon,
                      color: statusColor,
                      size: 24,
                    ),
                  );
                },
              ),
              const SizedBox(width: 16),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Text(
                    'Hybrid MCP RAG',
                    style: TextStyle(
                      color: Colors.white,
                      fontFamily: 'Outfit',
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    statusText,
                    style: TextStyle(
                      color: statusColor,
                      fontFamily: 'Inter',
                      fontSize: 14,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
