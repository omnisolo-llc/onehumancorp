import 'package:flutter/material.dart';
import 'dart:ui';

class EscalationStatusWidget extends StatelessWidget {
  final bool isCloudEscalated;
  final int activeTasks;

  const EscalationStatusWidget({
    Key? key,
    required this.isCloudEscalated,
    required this.activeTasks,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            color: const Color(0x0CFFFFFF), // rgba(255, 255, 255, 0.05)
            border: Border.all(color: const Color(0x19FFFFFF)), // rgba(255, 255, 255, 0.1)
            borderRadius: BorderRadius.circular(12),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                isCloudEscalated ? Icons.cloud : Icons.computer,
                color: Colors.white,
              ),
              const SizedBox(width: 12),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    isCloudEscalated ? 'Cloud Swarm' : 'Local Execution',
                    style: const TextStyle(
                      color: Colors.white,
                      fontFamily: 'Outfit',
                      fontWeight: FontWeight.w600,
                      fontSize: 16,
                    ),
                  ),
                  Text(
                    '$activeTasks Active Tasks',
                    style: const TextStyle(
                      color: Colors.white70,
                      fontFamily: 'Inter',
                      fontSize: 12,
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
