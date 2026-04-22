import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:ohc_app/models/dashboard.dart';
import 'package:ohc_app/models/agent.dart';

class ActiveAgentTraceWidget extends StatelessWidget {
  final List<Agent> activeAgents;

  const ActiveAgentTraceWidget({super.key, required this.activeAgents});

  @override
  Widget build(BuildContext context) {
    // Filter agents to only those that are running to act as 'active traces'
    final runningAgents = activeAgents.where((a) => a.isRunning).toList();

    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
        child: Container(
          padding: const EdgeInsets.all(24.0),
          decoration: BoxDecoration(
            color: const Color.fromRGBO(255, 255, 255, 0.05),
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: const Color.fromRGBO(255, 255, 255, 0.2)),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'Active Agent Traces',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              const SizedBox(height: 16),
              runningAgents.isEmpty
                  ? const Text(
                      'No active agents currently tracing.',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: Colors.white70,
                      ),
                    )
                  : Wrap(
                      spacing: 8.0,
                      runSpacing: 8.0,
                      children: runningAgents.map((agent) {
                        return Container(
                          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                          decoration: BoxDecoration(
                            color: Colors.blueAccent.withValues(alpha: 0.2),
                            borderRadius: BorderRadius.circular(12),
                            border: Border.all(color: Colors.blueAccent.withValues(alpha: 0.5)),
                          ),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              const Icon(Icons.memory, size: 16, color: Colors.blueAccent),
                              const SizedBox(width: 6),
                              Text(
                                agent.id ?? agent.name,
                                style: const TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 14,
                                  color: Colors.white,
                                ),
                              ),
                            ],
                          ),
                        );
                      }).toList(),
                    ),
            ],
          ),
        ),
      ),
    );
  }
}
