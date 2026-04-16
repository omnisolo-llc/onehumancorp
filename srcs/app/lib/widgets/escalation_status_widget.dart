import 'dart:ui';
import 'package:flutter/material.dart';

class EscalationStatusWidget extends StatelessWidget {
  final bool isEscalated;

  const EscalationStatusWidget({Key? key, required this.isEscalated}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(12),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0),
        child: Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.03),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: const Color.fromRGBO(255, 255, 255, 0.1),
              width: 1,
            ),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                'Task Status',
                style: const TextStyle(
                  fontFamily: 'Outfit',
                  color: Colors.white,
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 10),
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(
                    isEscalated ? Icons.cloud : Icons.computer,
                    color: isEscalated ? Colors.blue : Colors.green,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    isEscalated ? 'Cloud Orchestration' : 'Local SQLite',
                    style: const TextStyle(
                      fontFamily: 'Inter',
                      color: Colors.white70,
                      fontSize: 14,
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
