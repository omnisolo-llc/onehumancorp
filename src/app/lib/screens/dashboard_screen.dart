import 'package:flutter/material.dart';
import '../widgets/swarm_velocity_widget.dart';
import '../widgets/vector_memory_visualizer.dart';

class DashboardScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Swarm Dashboard', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            SwarmVelocityWidget(taskCompletionRate: 15.5, latencyMs: 25.0),
            const SizedBox(height: 16),
            VectorMemoryVisualizerWidget(vectorState: List.generate(1536, (i) => i * 0.001), isPulsing: true),
          ],
        ),
      ),
    );
  }
}
