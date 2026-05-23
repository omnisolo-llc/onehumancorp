import 'package:flutter/material.dart';
import '../widgets/vector_memory_visualizer.dart';

class SwarmMemoryScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Swarm Memory State', style: TextStyle(fontFamily: 'Outfit')),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: VectorMemoryVisualizerWidget(vectorState: List.generate(1536, (i) => i * 0.001), isPulsing: false),
        ),
      ),
    );
  }
}
