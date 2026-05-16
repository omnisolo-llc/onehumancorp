import 'dart:ui';
import 'package:flutter/material.dart';

class EscalationStatusWidget extends StatelessWidget {
  final bool isEscalated;

  const EscalationStatusWidget({Key? key, required this.isEscalated}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12.0),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
          decoration: BoxDecoration(
            color: Colors.white.withOpacity(0.05),
            borderRadius: BorderRadius.circular(12.0),
            border: Border.all(
              color: Colors.white.withOpacity(0.1),
              width: 1,
            ),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                isEscalated ? Icons.cloud_done : Icons.dns_rounded,
                color: isEscalated ? Colors.blueAccent : Colors.greenAccent,
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(
                isEscalated ? "Cloud Swarm" : "Local Execution",
                style: const TextStyle(
                  fontFamily: 'Inter',
                  color: Colors.white,
                  fontWeight: FontWeight.w500,
                  fontSize: 14,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
